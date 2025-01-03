pub mod light_pattern{

use clap::ValueEnum;
use strum_macros::EnumIter;

#[cfg(test)]
use strum::IntoEnumIterator;

#[derive(ValueEnum, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum LightPatternPublic {
    #[allow(non_camel_case_types)]
    r#static = 0x00,
    #[allow(non_camel_case_types)]
    breathing,
    #[allow(non_camel_case_types)]
    wave_left,
    #[allow(non_camel_case_types)]
    wave_right,
    #[allow(non_camel_case_types)]
    wave_up,
    #[allow(non_camel_case_types)]
    wave_down,
    #[allow(non_camel_case_types)]
    reactive,
    #[allow(non_camel_case_types)]
    reactive_on_input,
    #[allow(non_camel_case_types)]
    rainbow,
    #[allow(non_camel_case_types)]
    ripple,
    #[allow(non_camel_case_types)]
    ripple_on_input,
    #[allow(non_camel_case_types)]
    marquee,
    #[allow(non_camel_case_types)]
    raindrop,
    #[allow(non_camel_case_types)]
    aurora,
    #[allow(non_camel_case_types)]
    aurora_on_input,
    #[allow(non_camel_case_types)]
    gamemod,
    #[allow(non_camel_case_types)]
    spark,
    #[allow(non_camel_case_types)]
    spark_on_input,
    #[allow(non_camel_case_types)]
    music,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
#[repr(u8)]
pub enum LightPattern {
    #[allow(non_camel_case_types)]
    r#static = 0x01,
    #[allow(non_camel_case_types)]
    breathing = 0x02,
    #[allow(non_camel_case_types)]
    wave = 0x03,
    #[allow(non_camel_case_types)]
    reactive = 0x04,
    #[allow(non_camel_case_types)]
    rainbow = 0x05,
    #[allow(non_camel_case_types)]
    ripple = 0x06,
    #[allow(non_camel_case_types)]
    marquee = 0x09,
    #[allow(non_camel_case_types)]
    raindrop = 0x0a,
    #[allow(non_camel_case_types)]
    aurora = 0x0e,
    #[allow(non_camel_case_types)]
    gamemod = 0x0f,
    #[allow(non_camel_case_types)]
    spark = 0x11,
    #[allow(non_camel_case_types)]
    music = 0x21,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Direction {
    #[allow(non_camel_case_types)]
    none = 0x0,
    #[allow(non_camel_case_types)]
    right = 0x01,
    #[allow(non_camel_case_types)]
    left = 0x02,
    #[allow(non_camel_case_types)]
    up = 0x03,
    #[allow(non_camel_case_types)]
    down = 0x04,

}

#[derive(Debug)]
pub struct LightProfile {
    pub pattern: LightPattern,
    pub speed: u8,
    pub direction: Direction
}

pub const LIGHT_PROFILES: &'static [&'static LightProfile; 19] = &[
        &LightProfile{pattern: LightPattern::r#static, speed: 1, direction: Direction::none},
        &LightProfile{pattern: LightPattern::breathing, speed: 5, direction: Direction::none},
        // wave left, right, up, down
        &LightProfile{pattern: LightPattern::wave, speed: 10, direction: Direction::left},
        &LightProfile{pattern: LightPattern::wave, speed: 10, direction: Direction::right},
        &LightProfile{pattern: LightPattern::wave, speed: 10, direction: Direction::up},
        &LightProfile{pattern: LightPattern::wave, speed: 10, direction: Direction::down},
        &LightProfile{pattern: LightPattern::reactive, speed: 5, direction: Direction::none},
        &LightProfile{pattern: LightPattern::reactive, speed: 5, direction: Direction::right},
        &LightProfile{pattern: LightPattern::rainbow, speed: 0, direction: Direction::none},
        // ripple light on its own none or on input right
        &LightProfile{pattern: LightPattern::ripple, speed: 5, direction: Direction::none},
        &LightProfile{pattern: LightPattern::ripple, speed: 5, direction: Direction::right},
        &LightProfile{pattern: LightPattern::marquee, speed: 5, direction: Direction::none},
        &LightProfile{pattern: LightPattern::raindrop, speed: 5, direction: Direction::right},
        // aurora light on its own none or on input right
        &LightProfile{pattern: LightPattern::aurora, speed: 5, direction: Direction::none},
        &LightProfile{pattern: LightPattern::aurora, speed: 5, direction: Direction::right},
        &LightProfile{pattern: LightPattern::gamemod, speed: 5, direction: Direction::none},
        // spark light on its own none or on input right
        &LightProfile{pattern: LightPattern::spark, speed: 5, direction: Direction::none},
        &LightProfile{pattern: LightPattern::spark, speed: 5, direction: Direction::right},
        &LightProfile{pattern: LightPattern::music, speed: 9, direction: Direction::left},
];

#[test]
pub fn priv_test_profile_address() {
    let mut iter_exp = LightPattern::iter();
    let mut exp = None;
    for i in LightPatternPublic::r#static as usize..LightPatternPublic::music as usize {
        if ((LightPatternPublic::wave_left as usize) < i) && (i <= LightPatternPublic::wave_down as usize) {
        } else if ((LightPatternPublic::reactive as usize) < i) && (i <= LightPatternPublic::reactive_on_input as usize) {
        } else if ((LightPatternPublic::ripple as usize) < i) && (i <= LightPatternPublic::ripple_on_input as usize) {
        } else if ((LightPatternPublic::aurora as usize) < i) && (i <= LightPatternPublic::aurora_on_input as usize) {
        } else if ((LightPatternPublic::spark as usize) < i) && (i <= LightPatternPublic::spark_on_input as usize) {
        } else {
            exp = iter_exp.next();
        }
        assert_eq!(LIGHT_PROFILES[i].pattern, exp.unwrap());
    }
}

}

#[cfg(test)]
mod test {
    use crate::light_pattern::light_pattern::{priv_test_profile_address, };

    #[test]
    fn unit_test_profile_address() {
        priv_test_profile_address();
    }
}
