mod libtkc;
mod cmdline;

use clap::Parser;

use cmdline::Args;
use libtkc::device_handler::DeviceHandler;
use libtkc::rgbcolor::RGBColor;

fn main() {
    let  args = Args::parse();

#[cfg(feature = "completions")]
    if args.completions {
        return;
    }

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
