use crate::dimensions::Dimensions;

pub struct Yuv420 {
    pub dimensions: Dimensions,
    pub buffer: Vec<u8>,
}
