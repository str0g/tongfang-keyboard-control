use clap::Parser;

pub mod device_handler;

use device_handler::device_handler::{
    Brightness, ColorProfiles, DeviceHandler, LightPattern, RGBColor
};

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
    pattern: Option<LightPattern>,
}

fn main() {
    let dev = DeviceHandler::new();
    println!("{:#?}", dev.name);

    let  args = Args::parse();
    if !args.brigthness.is_none() {
        dev.set_brigthness(args.brigthness.unwrap());
    }
    if !args.color.is_none() {
        let color_profile = RGBColor::new(args.color.unwrap() as u32);
        dev.set_color(1, &color_profile);
    }
    if !args.pattern.is_none() {
        println!("@TODO");
        dev.set_profile(args.pattern.unwrap());
    }
}
