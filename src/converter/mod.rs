use crate::converter::cropping::crop_centered;
use crate::Dimensions;
use crate::buffer_2d::Buffer2d;
use scale::Scale;
use itertools::{MinMaxResult, Itertools};
use cpixel::Cpixel;
use std::cmp::Ordering;
use std::borrow::Cow;

pub mod cpixel;
mod scale;
mod cropping;

pub struct Converter {
    scale: Scale,
    output_constraints: Dimensions,
    input_image_dimensions: Dimensions,
    output_dimensions: Dimensions,
    maximize_contrast: bool,
    cpixel_size_ratio: f64,
    resize_type: ResizeType,
}

impl Converter {
    #[allow(dead_code)]
    pub fn convert_one<T: Into<u8> + Clone>(
        &mut self,
        image: &Buffer2d<T>,
    ) -> Buffer2d<Cpixel> {
        let mut image = Cow::Borrowed(image);
        if let ResizeType::Fill = self.resize_type {
            let width_to_crop = self.input_image_dimensions.width - self.scale.get_from_dimensions().width;
            let height_to_crop = self.input_image_dimensions.height - self.scale.get_from_dimensions().height;
            image = Cow::Owned(crop_centered(&image, width_to_crop, height_to_crop));
        }
        let mut buffer = self.scale.resize(&image);
        self.maybe_maximize_contrast(&mut buffer.buffer);
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
        output_constraints: Dimensions,
        input_image_dimensions: Dimensions,
        maximize_contrast: bool,
        resize_type: ResizeType
    ) -> Self {
        let cpixel_size_ratio: f64 = 2.0;
        let scaler_input_dimensions = Self::generate_scaler_dimensions(
            &input_image_dimensions,
            &output_constraints,
            &resize_type,
        );
        let output_dimensions = Self::generate_output_dimensions(
            &Dimensions {
                width: (input_image_dimensions.width as f64 * cpixel_size_ratio) as usize,
                ..input_image_dimensions
            },
            &output_constraints,
            &resize_type,
        );
        Self {
            scale: Scale::new(&scaler_input_dimensions, &output_dimensions),
            output_constraints,
            input_image_dimensions,
            output_dimensions,
            maximize_contrast,
            cpixel_size_ratio,
            resize_type,
        }
    }

    #[allow(dead_code)]
    pub fn resize_type(&self) -> ResizeType {
        self.resize_type
    }

    #[allow(dead_code)]
    pub fn cpixel_ratio(&self) -> f64 {
        self.cpixel_size_ratio
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
    pub fn output_dimensions(&self) -> &Dimensions {
        &self.output_dimensions
    }

    fn generate_scaler_dimensions(
        input_dimensions: &Dimensions,
        output_constraints: &Dimensions,
        resize_type: &ResizeType,
    ) -> Dimensions {
        if let ResizeType::Fill = resize_type {
            Self::fill_dimensions(input_dimensions, output_constraints)
        } else {
            *input_dimensions
        }
    }

    fn generate_output_dimensions(
        image_dimensions: &Dimensions,
        output_constraints: &Dimensions,
        resize_type: &ResizeType,
    ) -> Dimensions {
        if let ResizeType::Fit = resize_type {
            Dimensions::fit_with_locked_ratio(image_dimensions,
                                              output_constraints)
        } else {
            *output_constraints
        }
    }

    fn fill_dimensions(
        input_dimensions: &Dimensions,
        output_constraints: &Dimensions,
    ) -> Dimensions {
        use Ordering::*;

        let input_image_ratio = input_dimensions.ratio_width();
        let output_constraints_ratio = output_constraints.ratio_width();
        match input_image_ratio
            .partial_cmp(&output_constraints_ratio)
            .expect("Couldn't compare image dimension ratios")
        {
            Greater => {
                // Resize width.
                Dimensions {
                    height: input_dimensions.height,
                    width: (input_dimensions.width as f64 /
                        output_constraints_ratio) as usize,
                }
            }
            Less => {
                // Resize heigth.
                Dimensions {
                    height: (input_dimensions.height as f64 *
                        output_constraints_ratio) as usize,
                    width: input_dimensions.width,
                }
            }
            Equal => {
                // Do nothing.
                *input_dimensions
            }
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

#[derive(Copy, Clone, Debug)]
pub enum ResizeType {
    Fit,
    Fill,
    Stretch,
}
