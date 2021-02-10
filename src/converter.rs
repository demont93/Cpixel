use crate::{ImageBuffer, Cpixel, Dimensions};
use crate::cpixel::CpixelConverter;
use crate::pixel::{ToBrightness};
use itertools::{MinMaxResult, Itertools};

pub struct Converter {
    converter: CpixelConverter,
    cpixel_dimensions: Dimensions,
    output_constraints: Dimensions,
    input_image_dimensions: Dimensions,
    output_dimensions: Dimensions,
    maximize_contrast: bool,
}

impl Converter {
    #[allow(dead_code)]
    pub fn new(
        output_constraints: &Dimensions,
        input_image_dimensions: &Dimensions,
        cpixel_dimensions: &Dimensions,
        maximize_contrast: bool,
    ) -> Self {
        Self {
            converter: Default::default(),
            cpixel_dimensions: *cpixel_dimensions,
            output_constraints: *output_constraints,
            input_image_dimensions: *input_image_dimensions,
            output_dimensions: Self::generate_output_dimensions(
                input_image_dimensions,
                output_constraints,
                cpixel_dimensions,
            ),
            maximize_contrast,
        }
    }

    #[allow(dead_code)]
    pub fn maximizing_contrast_on(&self) -> bool {
        self.maximize_contrast
    }

    #[allow(dead_code)]
    pub fn constraints(&self) -> &Dimensions {
        &self.output_constraints
    }

    #[allow(dead_code)]
    pub fn image_settings(&self) -> &Dimensions {
        &self.input_image_dimensions
    }

    #[allow(dead_code)]
    pub fn cpixel_dimensions_settings(&self) -> &Dimensions {
        &self.cpixel_dimensions
    }

    #[allow(dead_code)]
    pub fn output_dimensions(&self) -> &Dimensions {
        &self.output_dimensions
    }

    #[allow(dead_code)]
    pub fn with_settings(
        self,
        output_constraints: &Dimensions,
        input_image_dimensions: &Dimensions,
        cpixel_dimensions: &Dimensions,
    ) -> Self {
        Converter {
            converter: self.converter,
            output_constraints: *output_constraints,
            input_image_dimensions: *input_image_dimensions,
            cpixel_dimensions: *cpixel_dimensions,
            output_dimensions: Self::generate_output_dimensions(
                input_image_dimensions,
                output_constraints,
                cpixel_dimensions,
            ),
            maximize_contrast: self.maximize_contrast,
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

    #[allow(dead_code)]
    pub fn convert_one<T: ToBrightness + Clone + Default>(
        &mut self,
        image: &ImageBuffer<T>,
    ) -> ImageBuffer<Cpixel> {
        let mut brightness_image = ImageBuffer::new(
            image.dimensions,
            image.buffer.iter().map(|x| x.to_brightness()).collect())
            .resize(&self.output_dimensions);
        self.maybe_maximize_contrast(&mut brightness_image.buffer);
        self.converter.convert(&brightness_image, &self.cpixel_dimensions)
    }

    fn maximize_contrast<'a>(
        pixels: impl Iterator<Item=&'a mut u8>, min: u8, max: u8,
    ) {
        let max_mult = u8::MAX / max;
        pixels.for_each(|x| {
            *x -= min;
            *x *= max_mult;
        })
    }
    fn maybe_maximize_contrast(&self, buffer: &mut Vec<u8>) {
        if self.maximize_contrast {
            let pixels = buffer.iter();
            if let MinMaxResult::MinMax(&min, &max) = pixels.minmax() {
                Self::maximize_contrast(
                    buffer.iter_mut(), min, max,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::image_buffer::ImageBuffer;
    use crate::converter::Converter;
    use crate::cpixel::Cpixel;
    use crate::dimensions::Dimensions;

    #[test]
    fn test_can_instance_converter() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let output_constraints = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        Converter::new(
            &output_constraints,
            &input_image_dimensions,
            &cpixel_dimensions,
            false,
        );
    }

    #[test]
    fn test_singleton_pixel_min() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let output_constraints = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let mut converter = Converter::new(
            &output_constraints,
            &input_image_dimensions,
            &cpixel_dimensions,
            false,
        );
        let image = ImageBuffer::new(input_image_dimensions, vec![0_u8]);
        let cpixel_image = converter.convert_one(&image);
        assert_eq!(cpixel_image.buffer, vec![Cpixel(' ')]);
    }

    #[test]
    fn test_singleton_pixel_max() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let output_constraints = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let mut converter: Converter = Converter::new(
            &output_constraints,
            &input_image_dimensions,
            &cpixel_dimensions,
            false,
        );
        let image = ImageBuffer::new(input_image_dimensions, vec![255_u8]);
        let cpixel_image = converter.convert_one(&image);
        assert_eq!(cpixel_image.buffer, vec![Cpixel('N')]);
    }
}
