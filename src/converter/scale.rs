use crate::{Dimensions, Buffer2d};
use std::cmp::Ordering;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};

pub struct Scale {
    grow_buffer: Buffer2d<usize>,
    shrink_buffer: Buffer2d<usize>,
    from_dimensions: Dimensions,
    to_dimensions: Dimensions,
    needs_grow: bool,
    needs_shrink: bool,
}

impl Scale {
    pub fn new(from_dimensions: &Dimensions, to_dimensions: &Dimensions) -> Self {
        let mut shrink_width = from_dimensions.width;
        let mut shrink_height = from_dimensions.height;
        let mut grow_width = from_dimensions.width;
        let mut grow_height = from_dimensions.height;
        let mut needs_grow;
        let mut needs_shrink;

        match to_dimensions.height.cmp(&from_dimensions.height) {
            Ordering::Less => {
                shrink_height = to_dimensions.height;
                grow_height = (from_dimensions.height as f64 / to_dimensions.height as f64).ceil() as usize * to_dimensions.height;
                if grow_height == from_dimensions.height {
                    needs_grow = false;
                } else {
                    needs_grow = true;
                }
                needs_shrink = true;
            }
            Ordering::Greater => {
                grow_height = to_dimensions.height;
                shrink_height = to_dimensions.height;
                needs_shrink = false;
                needs_grow = true;
            }
            Ordering::Equal => {
                needs_grow = false;
                needs_shrink = false;
            }
        };

        match to_dimensions.width.cmp(&from_dimensions.width) {
            Ordering::Less => {
                shrink_width = to_dimensions.width;
                grow_width = (from_dimensions.width as f64 / to_dimensions.width as f64).ceil() as usize * to_dimensions.width;
                if grow_width != from_dimensions.width {
                    needs_grow = true;
                }
                needs_shrink = true;
            }
            Ordering::Greater => {
                grow_width = to_dimensions.width;
                shrink_width = to_dimensions.width;
                needs_grow = true;
            }
            Ordering::Equal => {}
        };

        let grow_buffer = if needs_grow {
            Buffer2d::default_with_dimensions(&Dimensions { height: grow_height, width: grow_width })
        } else {
            Buffer2d::default()
        };
        let shrink_buffer = if needs_shrink {
            Buffer2d::default_with_dimensions(&Dimensions { height: shrink_height, width: shrink_width })
        } else {
            Buffer2d::default()
        };

        Self {
            grow_buffer,
            shrink_buffer,
            from_dimensions: from_dimensions.to_owned(),
            to_dimensions: to_dimensions.to_owned(),
            needs_grow,
            needs_shrink,
        }
    }

    #[allow(dead_code)]
    pub fn get_from_dimensions(&self) -> Dimensions {
        self.from_dimensions
    }

    #[allow(dead_code)]
    pub fn get_to_dimensions(&self) -> Dimensions {
        self.to_dimensions
    }

    pub fn resize<T: Into<u8> + Clone>(
        &mut self,
        buf: &Buffer2d<T>,
    ) -> Buffer2d<u8>
    {
        assert_eq!(
            buf.dimensions,
            self.from_dimensions,
            "Scale and buffer have different dimensions"
        );
        let mut buffer: &Buffer2d<usize> = &Buffer2d {
            buffer: buf.buffer.iter().map(|x| x.clone().into().into()).collect(),
            dimensions: buf.dimensions,
        };

        if self.needs_grow {
            buffer = Self::grow(buffer, &mut self.grow_buffer);
        }
        if self.needs_shrink {
            buffer = Self::shrink(buffer, &mut self.shrink_buffer);
        }
        Buffer2d {
            buffer: buffer.buffer.iter().map(|&n| n.try_into().unwrap()).collect(),
            dimensions: self.to_dimensions,
        }
    }

    fn shrink<'a>(
        buffer_to_resize: &Buffer2d<usize>,
        inner_buffer: &'a mut Buffer2d<usize>,
    ) -> &'a Buffer2d<usize> {
        inner_buffer.buffer.fill(0);
        let y_area = buffer_to_resize.dimensions.height / inner_buffer.dimensions.height;
        let x_area = buffer_to_resize.dimensions.width / inner_buffer.dimensions.width;
        let mut buf = buffer_to_resize.buffer.iter();
        for inner_row in inner_buffer.buffer.chunks_exact_mut(inner_buffer.dimensions.width) {
            for _ in 0..y_area {
                for inner_elem in inner_row.iter_mut() {
                    for _ in 0..x_area {
                        *inner_elem += buf.next().unwrap().clone();
                    }
                }
            }
        }

        inner_buffer.buffer.iter_mut().for_each(|elem| *elem = *elem / (x_area * y_area));
        inner_buffer
    }

    fn grow<'a>(
        buffer_to_resize: &Buffer2d<usize>,
        inner_buffer: &'a mut Buffer2d<usize>,
    ) -> &'a Buffer2d<usize> {
        let elements_to_interpolate_in_y = Self::elements_to_interpolate(
            buffer_to_resize.dimensions.height,
            inner_buffer.dimensions.height,
        );
        let elements_to_interpolate_in_x = Self::elements_to_interpolate(
            buffer_to_resize.dimensions.width,
            inner_buffer.dimensions.width,
        );

        let mut buffer_to_resize_chunked = buffer_to_resize.buffer
            .chunks_exact(buffer_to_resize.dimensions.width);
        let inner_buffer_indices = (0..inner_buffer.dimensions.total())
            .step_by(inner_buffer.dimensions.width);
        let zip_inner_buffer_indices_should_interpolate =
            inner_buffer_indices.zip(elements_to_interpolate_in_y.iter());

        for (index, &should_interpolate_row) in zip_inner_buffer_indices_should_interpolate {
            let inner_buffer_width = inner_buffer.dimensions.width;
            if should_interpolate_row {
                inner_buffer.buffer
                    .copy_within((index - inner_buffer_width)..index, index);
            } else {
                let mut col = buffer_to_resize_chunked.next().unwrap().iter();
                let zip_index_and_should_interpolate =
                    (index..index + inner_buffer_width).zip(elements_to_interpolate_in_x.iter());

                for (index, &should_interpolate_col) in zip_index_and_should_interpolate {
                    if should_interpolate_col {
                        inner_buffer.buffer[index] = inner_buffer.buffer[index - 1].clone();
                    } else {
                        inner_buffer.buffer[index] = col.next().unwrap().clone();
                    }
                }
            }
        }
        inner_buffer
    }

    fn elements_to_interpolate(
        initial_n: usize,
        final_n: usize,
    ) -> Vec<bool> {
        let stride = Self::calc_grow_stride(initial_n, final_n);
        let new_elems = (1..=final_n - initial_n).into_iter();
        let mut result = vec![false; final_n];
        new_elems
            .map(|elem| (elem as f64 * stride).ceil() as usize - 1)
            .for_each(|index| result[index] = true);
        result
    }

    fn calc_grow_stride(initial_n: usize, final_n: usize) -> f64 {
        let delta = final_n - initial_n;
        final_n as f64 / delta as f64
    }
}

impl Debug for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Scale{{ from_dimensions: {:?}, to_dimensions: {:?}, needs_grow: {:?}, needs_shrink: {:?} }}",
            self.from_dimensions,
            self.to_dimensions,
            self.needs_grow,
            self.needs_shrink
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::scale::Scale;
    use crate::{Dimensions, Buffer2d};

    #[test]
    fn test_grow_all_same() {
        let mut scale = Scale::new(&Dimensions { height: 5, width: 5 },
                                   &Dimensions { height: 10, width: 10 });
        let result = scale.resize(
            &Buffer2d::new(Dimensions { height: 5, width: 5 }, vec![35; 25])
        );
        assert_eq!(result.dimensions, Dimensions { height: 10, width: 10 });
        assert_eq!(result.buffer, vec![35; 100]);
    }

    #[test]
    fn test_from_different_values() {
        let mut scale = Scale::new(&Dimensions { height: 5, width: 2 },
                                   &Dimensions { height: 6, width: 1 });
        let result = scale.resize(
            &Buffer2d::new(
                Dimensions { height: 5, width: 2 },
                vec![10, 0, 8, 9, 6, 100, 97, 89, 34, 98],
            )
        );
        assert_eq!(result.dimensions, Dimensions { height: 6, width: 1 });
        assert_eq!(
            result.buffer,
            vec![5, 8, 53, 93, 66, 66]
        );
    }

    #[test]
    fn test_from_different_values_one_row() {
        let mut scale = Scale::new(&Dimensions { height: 5, width: 2 },
                                   &Dimensions { height: 6, width: 2 });
        let result = scale.resize(
            &Buffer2d::new(
                Dimensions { height: 5, width: 2 },
                vec![10, 0, 8, 9, 6, 1, 2, 3, 4, 9],
            )
        );
        assert_eq!(result.dimensions, Dimensions { height: 6, width: 2 });
        assert_eq!(
            result.buffer,
            vec![10, 0, 8, 9, 6, 1, 2, 3, 4, 9, 4, 9]
        );
    }

    #[test]
    fn test_more_than_one_interpolation_at_position() {
        let mut scale = Scale::new(&Dimensions { height: 1, width: 1 },
                                   &Dimensions { height: 3, width: 3 });
        let result = scale.resize(
            &Buffer2d::new(
                Dimensions { height: 1, width: 1 },
                vec![1],
            )
        );
        assert_eq!(result.dimensions, Dimensions { height: 3, width: 3 });
        assert_eq!(
            result.buffer,
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1]
        );
    }

    #[test]
    fn test_shrink_same() {
        let mut scale = Scale::new(&Dimensions { height: 10, width: 10 },
                                   &Dimensions { height: 5, width: 5 });
        let result = scale.resize(
            &Buffer2d::new(Dimensions { height: 10, width: 10 }, vec![35; 100])
        );
        assert_eq!(result.dimensions, Dimensions { height: 5, width: 5 });
        assert_eq!(result.buffer, vec![35; 25]);
    }

    #[test]
    fn test_shrink_different_values() {
        let mut scale = Scale::new(&Dimensions { height: 4, width: 4 },
                                   &Dimensions { height: 2, width: 2 });
        let result = scale.resize(
            &Buffer2d::new(Dimensions { height: 4, width: 4 }, vec![8, 4, 4, 8, 8, 4, 4, 8, 8, 4, 4, 8, 8, 4, 4, 8])
        );
        assert_eq!(result.dimensions, Dimensions { height: 2, width: 2 });
        assert_eq!(result.buffer, vec![6; 4]);
    }

    #[test]
    fn test_grow_shrink() {
        let mut scale = Scale::new(&Dimensions { height: 2, width: 5 },
                                   &Dimensions { height: 1, width: 6 });
        let result = scale.resize(
            &Buffer2d::new(
                Dimensions { height: 2, width: 5 },
                vec![10, 0, 8, 9, 6, 100, 97, 89, 34, 98],
            )
        );
        assert_eq!(result.dimensions, Dimensions { height: 1, width: 6 });
        assert_eq!(
            result.buffer,
            vec![55, 48, 48, 21, 52, 52]
        );
    }
}