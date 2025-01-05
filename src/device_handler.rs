use std::io::ErrorKind;
use futures_lite::future::block_on;
use nusb::{self, };

use crate::brightness::Brightness;
use crate::rgbcolor::RGBColor;
use crate::light_pattern::{LightPatternPublic, LIGHT_PROFILES};

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
