use std::iter::Sum;

use crate::{BitmapImage, Cpixel, Dimensions};
use crate::cpixel::CpixelConverter;

pub struct CpixelImageConverter<T> {
    converter: CpixelConverter<T>,
    cpixel_dimensions: Dimensions,
    output_constraints: Dimensions,
    input_image_dimensions: Dimensions,
    output_dimensions: Dimensions,
}

impl<PixelType> CpixelImageConverter<PixelType> {
    pub fn new(
        output_constraints: &Dimensions,
        input_image_dimensions: &Dimensions,
        cpixel_dimensions: &Dimensions,
    ) -> Self {
        Self {
            converter: Default::default(),
            cpixel_dimensions: *cpixel_dimensions,
            output_constraints: *output_constraints,
            input_image_dimensions: *input_image_dimensions,
            output_dimensions: Self::generate_output_dimensions(
                input_image_dimensions,
                output_constraints,
                cpixel_dimensions
            ),
        }
    }
    pub fn constraints(&self) -> &Dimensions {
        &self.output_constraints
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
        output_constraints: &Dimensions,
        input_image_dimensions: &Dimensions,
        cpixel_dimensions: &Dimensions,
    ) -> Self {
        CpixelImageConverter {
            converter: self.converter,
            output_constraints: *output_constraints,
            input_image_dimensions: *input_image_dimensions,
            cpixel_dimensions: *cpixel_dimensions,
            output_dimensions: Self::generate_output_dimensions(
                input_image_dimensions,
                output_constraints,
                cpixel_dimensions,
            ),
        }
    }

    fn generate_output_dimensions(
        image_dimensions: &Dimensions,
        output_constraints: &Dimensions,
        cpixel_dimensions: &Dimensions,
    ) -> Dimensions {
        let screen_in_pixels = Dimensions {
            height: output_constraints.height * cpixel_dimensions.height,
            width: output_constraints.width * cpixel_dimensions.width,
        };
        Dimensions::fit_with_locked_ratio(image_dimensions, &screen_in_pixels)
    }
}

impl<T: Into<u8> + Default + Copy + Sum + PartialOrd + From<u8>>
CpixelImageConverter<T> {
    pub fn convert(&mut self, image: &BitmapImage<T>) -> BitmapImage<Cpixel> {
        self.converter.convert(
            &image.resize(&self.output_dimensions),
            &self.cpixel_dimensions,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::bitmap_image::BitmapImage;
    use crate::converter::CpixelImageConverter;
    use crate::cpixel::Cpixel;
    use crate::dimensions::Dimensions;

    #[test]
    fn test_can_instance_converter() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let output_constraints = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        CpixelImageConverter::<u8>::new(
            &output_constraints,
            &input_image_dimensions,
            &cpixel_dimensions,
        );
    }

    #[test]
    fn test_singleton_pixel_min() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let output_constraints = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let mut converter = CpixelImageConverter::<u8>::new(
            &output_constraints,
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
        let output_constraints = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let mut converter: CpixelImageConverter<u8> = CpixelImageConverter::new(
            &output_constraints,
            &input_image_dimensions,
            &cpixel_dimensions,
        );
        let image = BitmapImage::new(input_image_dimensions, vec![255_u8]);
        let cpixel_image = converter.convert(&image);
        assert_eq!(cpixel_image.buffer, vec![Cpixel('N')]);
    }
}
