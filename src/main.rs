use clap::Parser;

pub mod brightness;
pub mod device_handler;
pub mod light_pattern;
pub mod rgbcolor;

use brightness::Brightness;
use device_handler::DeviceHandler;
use light_pattern::LightPatternPublic;
use rgbcolor::{RGBColor, ColorProfiles};

#[derive(Parser)]
struct Args {
    #[arg(
        short,
        long,
        value_enum)]
    brigthness: Option<Brightness>,
    #[arg(
        short,
        long,
        value_enum)]
    color: Option<ColorProfiles>,
    #[arg(
        short,
        long,
        value_enum)]
    pattern: Option<LightPatternPublic>,
}

fn main() {
    let  args = Args::parse();

    let dev = DeviceHandler::new();
    println!("{:#?}", dev.name);

    if !args.brigthness.is_none() {
        dev.set_brigthness(args.brigthness.unwrap());
    }
    if !args.color.is_none() {
        let color_profile = RGBColor::new(args.color.unwrap() as u32);
        dev.set_color(1, &color_profile);
    }
    if !args.pattern.is_none() {
        dev.set_profile(args.pattern.unwrap());
    }
}
