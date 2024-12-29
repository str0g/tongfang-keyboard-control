pub mod device_handler;

use device_handler::device_handler::{
    DeviceHandler,
    Brightness
};

fn main() {
   let dev = DeviceHandler::new();
   println!("{:#?}", dev.name);
   dev.set_brigthness(Brightness::MaxBrightness);
}
