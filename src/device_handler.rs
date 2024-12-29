
pub mod device_handler {

use std::ops::Range;

use futures_lite::future::block_on;
use nusb;
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
    white = 0xffffff,
    #[allow(non_camel_case_types)]
    red = 0xff0000,
    #[allow(non_camel_case_types)]
    blue = 0x0000ff,
    #[allow(non_camel_case_types)]
    green = 0x00ff00

}

struct SupportedDevice<'a> {
    name : &'a str,
    vendor_id: u16,
    product_id: u16,
}

const  SUPPORTED_DEVICES: &'static [&'static SupportedDevice; 2] = &[
    &SupportedDevice{vendor_id: 0x048d, product_id: 0x00, name: "place-holder"},
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
                Some(dev_info) => { return DeviceHandler { dev: dev_info.open().expect("failed to open device"), name: obj.name }; },
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

    pub fn set_one_color_to_all_areas(&self, color: &RGBColor) {
        for i in 1..5 {
            self.set_color(i, color);
        }
    }
}

}
