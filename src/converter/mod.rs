use crate::Dimensions;
use crate::buffer_2d::Buffer2d;
use scale::Scale;
use itertools::{MinMaxResult, Itertools};
use cpixel::Cpixel;
use std::cmp::{Ord, Ordering};

mod scale;
pub mod cpixel;

pub struct Converter {
    scale: Scale,
    output_constraints: Dimensions,
    input_image_dimensions: Dimensions,
    output_dimensions: Dimensions,
    maximize_contrast: bool,
    cpixel_size_ratio: f64,
}

impl Converter {
    #[allow(dead_code)]
    pub fn convert_one<T: Into<u8> + Clone>(
        &mut self,
        image: &Buffer2d<T>,
    ) -> Buffer2d<Cpixel> {
        let buffer = self.scale.resize(image);
        Buffer2d {
            buffer: buffer.buffer
                .iter()
                .map(|&n| Cpixel::from_brightness(n.into()))
                .collect(),
            dimensions: buffer.dimensions,
        }
    }
}

impl Converter {
    #[allow(dead_code)]
    pub fn new(
        output_constraints: &Dimensions,
        input_image_dimensions: &Dimensions,
        maximize_contrast: bool,
    ) -> Self {
        let cpixel_size_ratio: f64 = 2.0;
        let output_dimensions = Self::generate_output_dimensions(
            &Dimensions {
                width: (input_image_dimensions.width as f64 * cpixel_size_ratio) as usize,
                ..*input_image_dimensions
            },
            output_constraints,
        );
        Self {
            scale: Scale::new(input_image_dimensions, &output_dimensions),
            output_constraints: *output_constraints,
            input_image_dimensions: *input_image_dimensions,
            output_dimensions,
            maximize_contrast,
            cpixel_size_ratio,
        }
    }
}

impl Converter {
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
    pub fn output_dimensions(&self) -> &Dimensions {
        &self.output_dimensions
    }

    fn generate_output_dimensions(
        image_dimensions: &Dimensions,
        output_constraints: &Dimensions,
    ) -> Dimensions {
        let screen = Dimensions {
            height: output_constraints.height,
            width: output_constraints.width,
        };
        Dimensions::fit_with_locked_ratio(image_dimensions, &screen)
    }

    fn maximize_contrast<'a>(
        pixels: impl Iterator<Item=&'a mut u8>,
        min: u8,
        max: u8,
    ) {
        let max_mult = u8::MAX as f64 / (max - min) as f64;
        pixels.for_each(|x| {
            *x -= min;
            *x = (*x as f64 * max_mult).round() as u8;
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
