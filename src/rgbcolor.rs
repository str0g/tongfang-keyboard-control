pub mod rgbcolor {
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorProfiles {
    #[allow(non_camel_case_types)]
    blue = 0x0000ff,
    #[allow(non_camel_case_types)]
    cyan = 0x00ffff,
    #[allow(non_camel_case_types)]
    gold = 0xff4600,
    #[allow(non_camel_case_types)]
    green = 0x00ff00,
    #[allow(non_camel_case_types)]
    orange = 0xff2800,
    #[allow(non_camel_case_types)]
    pink = 0xff00c8,
    #[allow(non_camel_case_types)]
    purple = 0x8800c8,
    #[allow(non_camel_case_types)]
    red = 0xff0000,
    #[allow(non_camel_case_types)]
    white = 0xffffff,
    #[allow(non_camel_case_types)]
    yellow = 0xffff66,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    pub fn new(_in: u32) -> RGBColor {
        return RGBColor{r: (_in  >> 16 & 0xff) as u8, g: (_in >> 8 & 0xff) as u8, b: (_in & 0xff) as u8};
    }
}

}

#[cfg(test)]
mod tests {
    use crate::rgbcolor::rgbcolor::{RGBColor, ColorProfiles};

    #[test]
    fn test_conersion_white() {
        let exp = RGBColor {r:0xff , g: 0xff, b: 0xff};
        let out = RGBColor::new(ColorProfiles::white as u32);
        assert_eq!(out, exp);
    }

    #[test]
    fn test_conersion_red() {
        let exp = RGBColor {r:0xff , g: 0x00, b: 0x00};
        let out = RGBColor::new(ColorProfiles::red as u32);
        assert_eq!(out, exp);
    }

    #[test]
    fn test_conersion_green() {
        let exp = RGBColor {r:0x00 , g: 0xff, b: 0x00};
        let out = RGBColor::new(ColorProfiles::green as u32);
        assert_eq!(out, exp);
    }

    #[test]
    fn test_conersion_blue() {
        let exp = RGBColor {r:0x00 , g: 0x00, b: 0xff};
        let out = RGBColor::new(ColorProfiles::blue as u32);
        assert_eq!(out, exp);
    }
}

