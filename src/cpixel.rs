use crate::bitmap_image::BitmapImage;
use crate::dimensions::Dimensions;
use itertools::Itertools;
use std::iter::Sum;

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

pub struct CpixelConverter<T> {
    buf: Vec<T>,
}

impl<T> CpixelConverter<T> {
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

impl<T> Default for CpixelConverter<T> {
    fn default() -> Self {
        CpixelConverter { buf: vec![] }
    }
}

impl<T: Default> CpixelConverter<T> {
    pub fn resize(&mut self, new_len: usize) {
        self.buf.resize_with(new_len, T::default);
    }
}

impl<T: Into<u8> + Default + Sum + Copy + PartialOrd + From<u8>> CpixelConverter<T> {
    pub fn convert(
        &mut self,
        image: &BitmapImage<T>,
        cpixel_dimensions: &Dimensions,
    ) -> BitmapImage<Cpixel> {
        assert_eq!(image.dimensions.width % cpixel_dimensions.width, 0);
        assert_eq!(image.dimensions.height % cpixel_dimensions.height, 0);

        let new_dimensions = Dimensions {
            height: image.dimensions.height / cpixel_dimensions.height,
            width: image.dimensions.width / cpixel_dimensions.width,
        };

        // Resize the converter buffer, filling with default as necessary.
        self.resize(new_dimensions.total());

        let row_groups = image.buffer
            // Chunks of row_data.
            .chunks_exact(image.dimensions.width)
            // Chunks of rows of cpixel data.
            .map(|x| x.chunks_exact(cpixel_dimensions.width))
            // Chunks of cpixel rows.
            .chunks(cpixel_dimensions.height);

        let mut buf_slice: &mut [T];

        for (i, row_group) in row_groups.into_iter().enumerate() {
            let index = i * new_dimensions.width;
            buf_slice = &mut self.buf[index..];
            for row in row_group {
                for (a, b) in buf_slice.iter_mut().zip(row) {
                    *a = b.iter().copied().sum();
                }
            }
        }

        let new_buf = self.buf.iter()
            .map(|x| {
                debug_assert!(*x <= T::from(u8::MAX));
                let brightness = (*x).into() / cpixel_dimensions.total() as u8;
                Cpixel::from_brightness(brightness)
            });

        BitmapImage::new(new_dimensions, new_buf.collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitmap_image::BitmapImage;
    use crate::dimensions::Dimensions;

    #[test]
    fn test_convert_dimension_id_property() {
        let mut converter = CpixelConverter::default();
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let image = BitmapImage::new(Dimensions { height: 2, width: 4 },
                                     vec![0, 0, 0, 0, 0, 0, 0, 0]);
        let cpixel_image = converter.convert(&image, &cpixel_dimensions);
        assert_eq!(
            cpixel_image,
            BitmapImage { buffer: vec![Cpixel(' '); 8], dimensions: image.dimensions }
        );
    }

    #[test]
    fn test_convert_to_right_size() {}
}
