pub trait ToBrightness {
    fn to_brightness(&self) -> u8;
}


pub trait Pixel: ToBrightness {
    type DesaturatedPixel: Brightness;
    fn desaturate(&self) -> Self::DesaturatedPixel;
}


struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}


impl ToBrightness for RGB {
    fn to_brightness(&self) -> u8 {
        let sum = self.red as u16 + self.green as u16 + self.blue as u16;
        (sum as f64 / 3.0).round() as u8
    }
}


impl Pixel for RGB {
    type DesaturatedPixel = u8;

    fn desaturate(&self) -> Self::DesaturatedPixel {
        self.to_brightness()
    }
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
