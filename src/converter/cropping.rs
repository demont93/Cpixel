use crate::Buffer2d;
use crate::util::Point;

#[allow(dead_code)]
pub fn crop_area<T: Clone>(
    original_buffer: &Buffer2d<T>,
    top_left: &Point<usize>,
    bottom_right: &Point<usize>,
) -> Buffer2d<T> {
    crop(original_buffer, top_left, bottom_right)
}

pub fn crop_centered<T: Clone>(
    original_buffer: &Buffer2d<T>,
    width_to_crop: usize,
    height_to_crop: usize,
) -> Buffer2d<T> {
    let left = width_to_crop / 2;
    let right = original_buffer.dimensions.width - (width_to_crop - left) - 1;
    let top = height_to_crop / 2;
    let bottom = original_buffer.dimensions.height - (height_to_crop - top) - 1;
    crop(
        original_buffer,
        &Point { y: top, x: left },
        &Point { y: bottom, x: right },
    )
}

fn crop<T: Clone>(
    original_buffer: &Buffer2d<T>,
    top_left: &Point<usize>,
    bottom_right: &Point<usize>,
) -> Buffer2d<T> {
    let cropped = original_buffer.buffer
        .iter()
        .enumerate()
        .filter_map(|(flat_index, elem)| {
            let current_point = Point {
                y: flat_index / original_buffer.dimensions.width,
                x: flat_index % original_buffer.dimensions.width,
            };
            if current_point.any_lesser(top_left) ||
                current_point.any_greater(bottom_right)
            {
                None
            } else {
                Some(elem)
            }
        })
        .cloned()
        .collect();
    Buffer2d {
        dimensions: (*bottom_right + Point { y: 1, x: 1 } - *top_left).into(),
        buffer: cropped,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Dimensions;

    #[test]
    fn test_crop_id() {
        let buffer = Buffer2d {
            dimensions: Dimensions { height: 2, width: 5 },
            buffer: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        };
        let same_buffer = crop(&buffer, &Point { y: 0, x: 0 }, &Point { y: 1, x: 4 });
        assert_eq!(buffer, same_buffer);
    }

    #[test]
    fn test_crop_1_pixel() {
        let buffer = Buffer2d {
            dimensions: Dimensions { height: 3, width: 5 },
            buffer: vec![0; 15],
        };
        let buffer_1_pixel =
            crop(&buffer, &Point { y: 0, x: 0 }, &Point { y: 0, x: 0 });
        assert_eq!(buffer_1_pixel, Buffer2d::new(
            Dimensions { height: 1, width: 1 },
            vec![0],
        ));
    }

    #[test]
    fn test_crop_centered_even() {
        let buffer = Buffer2d {
            dimensions: Dimensions { height: 1, width: 4 },
            buffer: vec![255, 134, 89, 98],
        };
        let half_width_buffer = crop_centered(&buffer, 2, 0);
        assert_eq!(half_width_buffer, Buffer2d::new(
            Dimensions { height: 1, width: 2 },
            vec![134, 89],
        ));
    }

    #[test]
    fn test_crop_centered_uneven() {
        let buffer = Buffer2d {
            dimensions: Dimensions { height: 5, width: 1 },
            buffer: vec![255, 134, 89, 98, 2],
        };
        let half_width_buffer = crop_centered(&buffer, 0, 2);
        assert_eq!(half_width_buffer, Buffer2d::new(
            Dimensions { height: 3, width: 1 },
            vec![134, 89, 98],
        ));
    }
}