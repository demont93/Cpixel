use crate::dimensions::{Dimensions};
use crate::yuv::Yuv420;
use crate::pixel::{Pixel, Brightness};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Buffer2d<T> {
    pub dimensions: Dimensions,
    pub buffer: Vec<T>,
}

impl<T> Buffer2d<T> {
    pub fn new(dimensions: Dimensions, buffer: Vec<T>) -> Self {
        Buffer2d { dimensions, buffer }
    }
}

#[allow(dead_code)]
impl<T: Pixel> Buffer2d<T> {
    pub fn into_desaturated(self) -> Buffer2d<Brightness> {
        let Buffer2d { dimensions, buffer } = self;
        let buffer = buffer
            .into_iter()
            .map(|x| x.into_desaturated())
            .collect::<Vec<Brightness>>();
        Buffer2d::new(dimensions, buffer)
    }
}

impl<T> Default for Buffer2d<T> {
    fn default() -> Self {
        Buffer2d::new(Dimensions{ height: 0, width: 0 }, Vec::new())
    }
}

impl<T: Default + Clone> Buffer2d<T> {
    pub fn default_with_dimensions(dimensions: &Dimensions) -> Self {
        Self {
            dimensions: dimensions.to_owned(),
            buffer: vec![T::default(); dimensions.total()]
        }
    }
}

impl From<Yuv420> for Buffer2d<u8> {
    fn from(image: Yuv420) -> Self {
        let mut ret_val = Buffer2d {
            dimensions: image.dimensions,
            buffer: image.buffer,
        };
        ret_val.buffer.resize(
            (ret_val.dimensions.width * ret_val.dimensions.height) as usize,
            0,
        );
        ret_val
    }
}

impl<T: Copy> IntoIterator for Buffer2d<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}