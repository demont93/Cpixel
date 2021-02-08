use cpixel::{CpixelConverter};
use std::iter::Sum;

mod yuv;
mod bitmap_image;
mod dimensions;
mod cpixel;

pub use dimensions::{Dimensions, Dim};
pub use bitmap_image::{BitmapImage};
pub use cpixel::Cpixel;

pub struct CpixelImageConverter<T> {
    converter: CpixelConverter<T>,
    cpixel_dimensions: Dimensions,
    screen_dimensions: Dimensions,
    input_image_dimensions: Dimensions,
    output_dimensions: Dimensions,
}

impl<PixelType> CpixelImageConverter<PixelType> {
    pub fn new(
        screen_dimensions: &Dimensions,
        input_image_dimensions: &Dimensions,
        cpixel_dimensions: &Dimensions,
    ) -> Self {
        Self {
            converter: Default::default(),
            cpixel_dimensions: *cpixel_dimensions,
            screen_dimensions: *screen_dimensions,
            input_image_dimensions: *input_image_dimensions,
            output_dimensions: Self::generate_output_dimensions(
                input_image_dimensions,
                screen_dimensions,
            ),
        }
    }
    pub fn screen_settings(&self) -> &Dimensions {
        &self.screen_dimensions
    }

    pub fn image_settings(&self) -> &Dimensions {
        &self.input_image_dimensions
    }

    pub fn cpixel_dimensions_settings(&self) -> &Dimensions {
        &self.cpixel_dimensions
    }

    pub fn output_dimensions(&self) -> &Dimensions {
        &self.output_dimensions
    }

    pub fn with_settings(
        self,
        screen_dimensions: &Dimensions,
        input_image_dimensions: &Dimensions,
        cpixel_dimensions: &Dimensions,
    ) -> Self {
        CpixelImageConverter {
            converter: self.converter,
            screen_dimensions: *screen_dimensions,
            input_image_dimensions: *input_image_dimensions,
            cpixel_dimensions: *cpixel_dimensions,
            output_dimensions: Self::generate_output_dimensions(
                input_image_dimensions,
                screen_dimensions,
            ),
        }
    }

    fn generate_output_dimensions(
        image_dimensions: &Dimensions,
        screen_dimensions: &Dimensions,
    ) -> Dimensions {
        Dimensions::closest_best_size(image_dimensions, screen_dimensions)
    }
}

impl<T: Into<u8> + Default + Copy + Sum> CpixelImageConverter<T> {
    pub fn convert(&mut self, image: &BitmapImage<T>) -> BitmapImage<Cpixel> {
        self.converter.convert(&image, &self.cpixel_dimensions)
    }
}

#[cfg(test)]
mod tests {
    use crate::dimensions::Dimensions;
    use crate::cpixel::Cpixel;
    use crate::CpixelImageConverter;
    use crate::bitmap_image::BitmapImage;

    #[test]
    fn test_can_instance_converter() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let screen_dimensions = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        CpixelImageConverter::<u8>::new(
            &screen_dimensions,
            &input_image_dimensions,
            &cpixel_dimensions,
        );
    }

    #[test]
    fn test_singleton_pixel_min() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let screen_dimensions = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let mut converter = CpixelImageConverter::<u8>::new(
            &screen_dimensions,
            &input_image_dimensions,
            &cpixel_dimensions,
        );
        let image = BitmapImage::new(input_image_dimensions, vec![0_u8]);
        let cpixel_image = converter.convert(&image);
        assert_eq!(cpixel_image.buffer, vec![Cpixel(' ')]);
    }

    #[test]
    fn test_singleton_pixel_max() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let screen_dimensions = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let mut converter: CpixelImageConverter<u8> = CpixelImageConverter::new(
            &screen_dimensions,
            &input_image_dimensions,
            &cpixel_dimensions,
        );
        let image = BitmapImage::new(input_image_dimensions, vec![255_u8]);
        let cpixel_image = converter.convert(&image);
        assert_eq!(cpixel_image.buffer, vec![Cpixel('N')]);
    }
}
