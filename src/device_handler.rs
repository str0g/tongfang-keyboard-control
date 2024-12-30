
pub mod device_handler {

use std::io::ErrorKind;
use futures_lite::future::block_on;
use nusb::{self, };
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Brightness {
    #[allow(non_camel_case_types)]
    off = 0x00,
    #[allow(non_camel_case_types)]
    low = 0x08,
    #[allow(non_camel_case_types)]
    med = 0x16,
    #[allow(non_camel_case_types)]
    high = 0x24,
    #[allow(non_camel_case_types)]
    max = 0x32,
}

#[derive(Debug)]
pub struct RGBColor {
    r: u8,
    g: u8,
    b: u8,
}

impl RGBColor {
    pub fn new(_in: u32) -> RGBColor {
        return RGBColor{r: (_in  >> 16 & 0xff) as u8, g: (_in >> 8 & 0xff) as u8, b: (_in & 0xff) as u8};
    }
}

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

#[derive(ValueEnum, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum LightPattern {
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
    music = 0x22,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Direction {
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
struct LightProfile {
    pattern: LightPattern,
    speed: u8,
    direction: Direction
}

const LIGHT_PROFILES: &'static [&'static LightProfile; 18] = &[
        &LightProfile{pattern: LightPattern::r#static, speed: 1, direction: Direction::none},
        &LightProfile{pattern: LightPattern::breathing, speed: 5, direction: Direction::none},//1
        // wave left, right, up, down
        &LightProfile{pattern: LightPattern::wave, speed: 10, direction: Direction::left},//2
        &LightProfile{pattern: LightPattern::wave, speed: 10, direction: Direction::right},
        &LightProfile{pattern: LightPattern::wave, speed: 10, direction: Direction::up},
        &LightProfile{pattern: LightPattern::wave, speed: 10, direction: Direction::down},
        &LightProfile{pattern: LightPattern::rainbow, speed: 0, direction: Direction::none},
        &LightProfile{pattern: LightPattern::reactive, speed: 5, direction: Direction::none},//7
        // ripple light on its own none or on input right
        &LightProfile{pattern: LightPattern::ripple, speed: 5, direction: Direction::none},//8
        &LightProfile{pattern: LightPattern::ripple, speed: 5, direction: Direction::right},
        &LightProfile{pattern: LightPattern::marquee, speed: 5, direction: Direction::none},
        &LightProfile{pattern: LightPattern::raindrop, speed: 5, direction: Direction::right},//11
        // aurora light on its own none or on input right
        &LightProfile{pattern: LightPattern::aurora, speed: 5, direction: Direction::none},//12
        &LightProfile{pattern: LightPattern::aurora, speed: 5, direction: Direction::right},
        &LightProfile{pattern: LightPattern::gamemod, speed: 5, direction: Direction::none},//14
        // spark light on its own none or on input right
        &LightProfile{pattern: LightPattern::spark, speed: 5, direction: Direction::none},//15
        &LightProfile{pattern: LightPattern::spark, speed: 5, direction: Direction::right},
        &LightProfile{pattern: LightPattern::music, speed: 9, direction: Direction::left},
];

struct SupportedDevice<'a> {
    name : &'a str,
    vendor_id: u16,
    product_id: u16,
}

const SUPPORTED_DEVICES: &'static [&'static SupportedDevice; 1] = &[
    &SupportedDevice{vendor_id: 0x048d, product_id: 0xce00, name: "xmg neo 17 e21"}
];

pub struct DeviceHandler {
    dev: nusb::Device,
    pub name: &'static str,
}

impl DeviceHandler {
    pub fn new() -> DeviceHandler {
        let rc = None;
        let mut i = 0;
        while i < SUPPORTED_DEVICES.len() {
            let obj = &SUPPORTED_DEVICES[i];
            let _dev_info = match nusb::list_devices().unwrap()
                            .find(|dev| dev.vendor_id() == obj.vendor_id && dev.product_id() == obj.product_id) {
                Some(dev_info) => {
                        let _dev = dev_info.open().expect("failed to open device");
                        if _dev.attach_kernel_driver(1).is_err_and(|_dev| _dev.kind() == ErrorKind::ResourceBusy) {
                            _dev.detach_kernel_driver(1).expect("initial attachment failed");
                        } else {
                            _dev.detach_kernel_driver(1).expect("restarting attachment");
                        }
                    return DeviceHandler { dev: _dev, name: obj.name };
                    },
                None => { println!("Skipping {}", obj.name); },
            };
            i += 1;
        }
        return rc.expect("No supported devices has been found");
    }

    fn send_request(&self, _data: &[u8] ) {
        block_on(self.dev.control_out(nusb::transfer::ControlOut {
                    control_type: nusb::transfer::ControlType::Class,
                    recipient: nusb::transfer::Recipient::Interface,
                    request: 0x09,
                    value: 0x300,
                    index: 0x01,
                    data: _data,
                })).status.expect("communication error");
    }

    pub fn set_brigthness(&self, brightness: Brightness) {
        self.send_request(&[0x08, 0x02, 0x01, 0x05, brightness as u8, 0x08, 0x00, 0x00]);
    }

    pub fn set_color(&self, area:u8, color: &RGBColor) {
        self.send_request(&[0x14, 0x00, area, color.r, color.g, color.b, 0x00, 0x00]);
    }

    pub fn set_profile(&self, profile: LightPatternPublic) {
        let brightness = Brightness::max as u8;
        let profile = LIGHT_PROFILES[profile as usize];
//        println!("{:#?}", profile);
        self.send_request(&[0x08, 0x02,
                profile.pattern as u8,
                profile.speed,
                brightness,
                0x08,
                profile.direction as u8,
                0x01]);
    }
}

}
