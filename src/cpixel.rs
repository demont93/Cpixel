use crate::image_buffer::ImageBuffer;
use crate::dimensions::Dimensions;
use itertools::{Itertools, IntoChunks};
use std::iter::{Iterator};
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Cpixel(pub char);

impl Cpixel {
    pub fn from_brightness(brightness: u8) -> Self {
        static CHARS: &'static [char] = &[
            ' ', '`', '.', '\'', '_', '~', '"', '^', 'r', '!', '/', '(', ')',
            '?', 'i', 't', 'c', 'j', '=', '7', 'Y', 'J', '}', '1', 'o', '%',
            'e', 'V', 'S', 'F', '4', 'k', '5', 'O', 'q', 'd', 'p', 'Q', 'E',
            '9', 'H', 'g', 'R', 'm', 'W', '@', 'B', 'N'
        ];
        static BRIGHTNESS: &'static [u8] = &[
            0_u8, 5_u8, 14_u8, 23_u8, 32_u8, 45_u8, 58_u8, 64_u8, 69_u8, 75_u8,
            82_u8, 92_u8, 97_u8, 101_u8, 108_u8, 116_u8, 119_u8, 123_u8, 127_u8,
            131_u8, 134_u8, 138_u8, 142_u8, 145_u8, 151_u8, 156_u8, 160_u8,
            164_u8, 168_u8, 171_u8, 177_u8, 184_u8, 190_u8, 193_u8, 197_u8,
            201_u8, 204_u8, 208_u8, 212_u8, 216_u8, 219_u8, 223_u8, 227_u8,
            230_u8, 236_u8, 243_u8, 249_u8, 253_u8
        ];
        let index = BRIGHTNESS.iter().rposition(|x| *x <= brightness).unwrap();
        return Cpixel(CHARS[index]);
    }
}

impl Display for Cpixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Cpixel(c) = self;
        write!(f, "{}", *c)
    }
}

pub struct CpixelConverter {
    buf: Vec<usize>,
}

impl CpixelConverter {
    #[allow(dead_code)]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    #[allow(dead_code)]
    pub fn shrink(&mut self, n: usize) {
        if n > self.buf.capacity() { return; }
        self.buf.truncate(n);
        self.buf.shrink_to_fit();
    }
}

impl CpixelConverter {
    pub fn resize_buf(&mut self, new_len: usize) {
        self.buf.resize_with(new_len, usize::default);
    }

    pub fn convert(
        &mut self,
        image: &ImageBuffer<u8>,
        cpixel_dimensions: &Dimensions,
    ) -> ImageBuffer<Cpixel> {
        assert_eq!(image.dimensions.width % cpixel_dimensions.width, 0);
        assert_eq!(image.dimensions.height % cpixel_dimensions.height, 0);

        let new_dimensions = Self::generate_new_dimensions(
            &image.dimensions, cpixel_dimensions,
        );
        // Resize the converter buffer, filling with default as necessary.
        self.resize_buf(new_dimensions.total());
        self.reset_buf();

        let pixel_groups = self.partition_cpixels(image, cpixel_dimensions);
        self.store_cpixel_brightness_sum(pixel_groups, &new_dimensions);
        let new_buf =  self.collect_from_buf(cpixel_dimensions);
        ImageBuffer::new(new_dimensions, new_buf)
    }

    fn generate_new_dimensions(
        image_dimensions: &Dimensions,
        cpixel_dimensions: &Dimensions,
    ) -> Dimensions {
        Dimensions {
            height: image_dimensions.height / cpixel_dimensions.height,
            width: image_dimensions.width / cpixel_dimensions.width,
        }
    }

    fn store_cpixel_brightness_sum<'a>(
        &mut self,
        row_groups: IntoChunks<impl Iterator<Item=impl Iterator<Item=&'a [u8]> + 'a> + 'a>,
        new_dimensions: &Dimensions,
    ) {
        for (i, row_group) in row_groups.into_iter().enumerate() {
            let index = i * new_dimensions.width;
            let buf_slice = &mut self.buf[index..];
            for row in row_group {
                for (a, b) in buf_slice.iter_mut().zip(row) {
                    *a += b.iter().map(|x| *x as usize).sum::<usize>();
                }
            }
        }
    }

    fn collect_from_buf(&self, cpixel_dimensions: &Dimensions) -> Vec<Cpixel> {
        let pixels_in_cpixel = cpixel_dimensions.total();
        self.buf.iter()
            .map(|x| {
                let average = (*x / pixels_in_cpixel) as u8;
                Cpixel::from_brightness(average)
            })
            .collect()
    }

    fn partition_cpixels<'a, 'b>(
        &self,
        image: &'a ImageBuffer<u8>,
        cpixel_dimensions: &'b Dimensions,
    ) -> IntoChunks<impl Iterator<Item=impl Iterator<Item=&'a [u8]> + 'a> + 'a> {
        let width = cpixel_dimensions.width;
        let height = cpixel_dimensions.height;
        image.buffer
            // Chunks of row_data.
            .chunks_exact(image.dimensions.width)
            // Chunks of rows of cpixel data.
            .map(move |x| x.chunks_exact(width))
            // Chunks of cpixel rows.
            .chunks(height)
    }

    fn reset_buf(&mut self) {
        self.buf.fill(0)
    }
}

impl Default for CpixelConverter {
    fn default() -> Self {
        CpixelConverter { buf: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image_buffer::ImageBuffer;
    use crate::dimensions::Dimensions;

    #[test]
    fn test_convert_dimension_id_property() {
        let mut converter = CpixelConverter::default();
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let image = ImageBuffer::new(Dimensions { height: 2, width: 4 },
                                     vec![0, 0, 0, 0, 0, 0, 0, 0]);
        let cpixel_image = converter.convert(&image, &cpixel_dimensions);
        assert_eq!(
            cpixel_image,
            ImageBuffer { buffer: vec![Cpixel(' '); 8], dimensions: image.dimensions }
        );
    }

    #[test]
    fn test_convert_to_right_size() {}
}
