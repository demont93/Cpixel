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
    let right = original_buffer.dimensions.width - (width_to_crop - left);
    let top = height_to_crop / 2;
    let bottom = original_buffer.dimensions.height - (height_to_crop - top);
    crop(original_buffer, &Point{ y: top, x: left }, &Point{ y: bottom, x: right })
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
        dimensions: (*bottom_right - *top_left).into(),
        buffer: cropped,
    }
}
