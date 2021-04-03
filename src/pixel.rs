use std::ops::{Deref, DerefMut};

pub trait Pixel {
    fn into_desaturated(self) -> Brightness;
}

pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel for RGB {
    fn into_desaturated(self) -> Brightness {
        (((self.red as u16 + self.green as u16 + self.blue as u16) / 3) as u8)
            .into()
    }
}

impl Pixel for Brightness {
    fn into_desaturated(self) -> Brightness {
        self
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Brightness(u8);

impl Brightness {
    #[allow(dead_code)]
    fn min() -> Self {
        u8::MIN.into()
    }

    #[allow(dead_code)]
    fn max() -> Self {
        u8::MAX.into()
    }

    #[allow(dead_code)]
    fn average(&self, rhs: &Self) -> Self {
        ((*self.deref() as u16 + *rhs.deref() as u16 / 2) as u8).into()
    }

    #[allow(dead_code)]
    fn to_byte(&self) -> u8 {
        *self.deref()
    }
}

impl Deref for Brightness {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Brightness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u8> for Brightness {
    fn from(byte: u8) -> Self {
        Brightness(byte)
    }
}
