
pub trait ToBrightness {
    fn to_brightness(&self) -> u8;
}

pub trait Pixel: ToBrightness {
    type DesaturatedPixel: Brightness;
    fn desaturate(&self) -> Self::DesaturatedPixel;
}

pub trait Brightness: ToBrightness {
    fn min() -> Self;
    fn max() -> Self;
    fn average(&self, rhs: &Self) -> Self;
    fn to_byte(&self) -> u8;
}

impl ToBrightness for u8 {
    fn to_brightness(&self) -> u8 {
        self.to_byte()
    }
}

impl Brightness for u8 {
    fn min() -> Self {
        u8::MIN
    }

    fn max() -> Self {
        u8::MAX
    }

    fn average(&self, rhs: &Self) -> Self {
        (*self as u16 + *rhs as u16 / 2) as u8
    }

    fn to_byte(&self) -> u8 {
        *self
    }
}
