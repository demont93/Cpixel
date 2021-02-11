use std::cmp::Ordering;

use crate::dimensions::{Dim, Dimensions};
use crate::yuv::Yuv420;
use crate::pixel::{Pixel};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageBuffer<T> {
    pub dimensions: Dimensions,
    pub buffer: Vec<T>,
}

impl<T> ImageBuffer<T> {
    pub fn new(dimensions: Dimensions, buffer: Vec<T>) -> Self {
        ImageBuffer { dimensions, buffer }
    }
}

impl<T: Clone + Default> ImageBuffer<T> {
    pub fn resize(&self, dimensions: &Dimensions) -> ImageBuffer<T> {
        let new_buf: Vec<T> = vec![Default::default(); dimensions.total()];
        match dimensions.partial_cmp(&self.dimensions) {
            None => panic!("Shouldn't reach here."),
            Some(Ordering::Equal) => {
                self.clone()
            }
            Some(Ordering::Less) => {
                self.shrink(*dimensions, new_buf)
            }
            Some(Ordering::Greater) => {
                self.grow(*dimensions, new_buf)
            }
        }
    }

    pub fn resize_locked(&self, size: Dim) -> ImageBuffer<T> {
        let new_dimensions = self.dimensions.locked_ratio_resize(&size);
        self.resize(&new_dimensions)
    }

    fn shrink(&self, new_dimensions: Dimensions, mut new_buffer: Vec<T>) -> Self {
        let height_delta = self.dimensions.height - new_dimensions.height;
        let width_delta = self.dimensions.width - new_dimensions.width;

        let col_stride = 1.0 + new_dimensions.height as f64 / height_delta as f64;
        let row_stride = 1.0 + new_dimensions.width as f64 / width_delta as f64;

        let mut one_col = vec![false; self.dimensions.height];
        (0..height_delta).for_each(|x| {
            one_col[(x as f64 * col_stride) as usize] = true;
        });

        let mut one_row = vec![false; self.dimensions.width];
        (0..width_delta).for_each(|x| {
            one_row[(x as f64 * row_stride) as usize] = true;
        });

        let mut it = self.buffer.iter();
        let mut nb_it = new_buffer.iter_mut();

        one_col.iter()
            .for_each(|row_interpolated| {
                if *row_interpolated {
                    for _ in 0..self.dimensions.width { it.next(); }
                } else {
                    one_row.iter()
                        .for_each(|col_interpolated| {
                            if *col_interpolated {
                                it.next();
                            } else {
                                *nb_it.next().unwrap() = it.next().unwrap().to_owned();
                            }
                        })
                }
            });
        ImageBuffer::new(new_dimensions, new_buffer)
    }

    fn grow(&self, new_dimensions: Dimensions, mut new_buffer: Vec<T>) -> Self {
        let height_delta = new_dimensions.height - self.dimensions.height;
        let width_delta = new_dimensions.width - self.dimensions.width;
        let col_stride = 1.0 + self.dimensions.height as f64 / height_delta as f64;
        let row_stride = 1.0 + self.dimensions.width as f64 / width_delta as f64;
        let mut it = self.buffer.iter();
        let mut one_row = vec![false; new_dimensions.width];
        (0..width_delta).for_each(|x| {
            one_row[(x as f64 * row_stride) as usize + 1] = true;
        });
        let mut one_col = vec![false; new_dimensions.height];
        (0..height_delta).for_each(|x| {
            one_col[(x as f64 * col_stride) as usize + 1] = true;
        });

        one_col.iter()
            .enumerate()
            .for_each(|(y, interpolated)| {
                if *interpolated {
                    self.interpolate_row(y, &new_dimensions, &mut new_buffer)
                } else {
                    one_row.iter()
                        .enumerate()
                        .for_each(|(x, interpolated)| {
                            let index = y * new_dimensions.width + x;
                            if *interpolated {
                                self.interpolate_col(y, x, &new_dimensions, &mut new_buffer)
                            } else {
                                new_buffer[index] = it.next().unwrap().to_owned();
                            }
                        })
                }
            });
        ImageBuffer::new(new_dimensions, new_buffer)
    }

    fn interpolate_col(
        &self,
        row: usize, col: usize,
        new_dimensions: &Dimensions, new_buffer: &mut [T],
    ) {
        let index = row * new_dimensions.width + col;
        new_buffer[index] = new_buffer[index - 1].clone();
    }

    fn interpolate_row(
        &self, row: usize, new_dimensions: &Dimensions, new_buffer: &mut [T],
    ) {
        let index = row * new_dimensions.width;
        let from = index - new_dimensions.width;
        let to = index + new_dimensions.width;
        let (left, right) = new_buffer[from..to]
            .split_at_mut(new_dimensions.width);
        right.clone_from_slice(left);
    }
}

impl<T: Pixel> ImageBuffer<T> {
    pub fn desaturate(&self) -> ImageBuffer<T::DesaturatedPixel> {
        ImageBuffer {
            buffer: self.buffer.iter().map(|x| x.desaturate()).collect(),
            dimensions: self.dimensions,
        }
    }
}

impl From<Yuv420> for ImageBuffer<u8> {
    fn from(image: Yuv420) -> Self {
        let mut ret_val = ImageBuffer {
            dimensions: image.dimensions,
            buffer: image.buffer,
        };
        ret_val.buffer.resize(
            (ret_val.dimensions.width * ret_val.dimensions.height) as usize,
            0,
        );
        ret_val
    }
}

impl<T: Copy> IntoIterator for ImageBuffer<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_grow_same_value() {
        let buf = vec![10; 10];
        let dimensions = Dimensions { height: 1, width: 10 };
        let image = ImageBuffer::new(dimensions, buf);
        let image = image.resize_locked(Dim::Width(15));
        assert_eq!(image.buffer, vec![10; 15]);
        assert_eq!(image.dimensions, Dimensions { height: 1, width: 15 });
    }

    #[test]
    fn test_2d_grow() {
        let buf = vec![10; 40];
        let dimensions = Dimensions { height: 4, width: 10 };
        let image = ImageBuffer::new(dimensions, buf);
        let image = image.resize_locked(Dim::Width(15));
        assert_eq!(image.buffer, vec![10; 90]);
        assert_eq!(image.dimensions, Dimensions { height: 6, width: 15 });
    }

    #[test]
    fn test_1_row_different_values() {
        let buf = (1..=10).collect::<Vec<usize>>();
        let dimensions = Dimensions { height: 1, width: 10 };
        let image = ImageBuffer::new(dimensions, buf);
        let image = image.resize_locked(Dim::Width(15));
        assert_eq!(image.buffer, vec![
            1, 1, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 9, 9, 10
        ]);
        assert_eq!(image.dimensions, Dimensions { height: 1, width: 15 });
    }

    #[test]
    fn test_many_rows_interpolation() {
        let buf = (1..=30).collect::<Vec<usize>>();
        let dimensions = Dimensions { height: 3, width: 10 };
        let image = ImageBuffer::new(dimensions, buf);
        let image = image.resize_locked(Dim::Width(15));
        assert_eq!(image.buffer, vec![
            1, 1, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 9, 9, 10,
            1, 1, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 9, 9, 10,
            11, 11, 12, 13, 13, 14, 15, 15, 16, 17, 17, 18, 19, 19, 20,
            21, 21, 22, 23, 23, 24, 25, 25, 26, 27, 27, 28, 29, 29, 30
        ]);
        assert_eq!(image.dimensions, Dimensions { height: 4, width: 15 });
    }

    #[test]
    fn test_grow_2_to_7() {
        let buf = vec![10, 11];
        let dimensions = Dimensions { height: 1, width: 2 };
        let image = ImageBuffer::new(dimensions, buf);
        let image = image.resize_locked(Dim::Width(7));
        assert_eq!(image.buffer, vec![
            10, 10, 10, 10, 11, 11, 11, 10, 10, 10, 10, 11, 11, 11, 10, 10, 10,
            10, 11, 11, 11
        ]);
        assert_eq!(image.dimensions, Dimensions { height: 3, width: 7 });
    }

    #[test]
    fn test_shrink_simple() {
        let buf = vec![1; 10];
        let dimensions = Dimensions { height: 1, width: 10 };
        let image = ImageBuffer::new(dimensions, buf);
        let image = image.resize_locked(Dim::Width(5));
        assert_eq!(image.buffer, vec![1; 5]);
        assert_eq!(image.dimensions, Dimensions { height: 1, width: 5 });
    }

    #[test]
    fn test_shrink_2d() {
        let buf = (1..=40).collect();
        let dimensions = Dimensions { height: 4, width: 10 };
        let image = ImageBuffer::new(dimensions, buf);
        let image = image.resize_locked(Dim::Width(3));
        assert_eq!(image.buffer, vec![34, 37, 40]);
        assert_eq!(image.dimensions, Dimensions { height: 1, width: 3 });
    }
}