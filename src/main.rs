use clap::Parser;

pub mod device_handler;

use device_handler::device_handler::{
    DeviceHandler,
    Brightness
};

#[derive(Parser)]
struct Args {
    #[arg(
        short,
        long,
        value_enum)]
    brigthness: Option<Brightness>,
}

fn main() {
    let dev = DeviceHandler::new();
    println!("{:#?}", dev.name);

    let  args = Args::parse();
    println!("args: {:#?}", args.brigthness);
    if !args.brigthness.is_none() {
        dev.set_brigthness(args.brigthness.unwrap());
    }
}
