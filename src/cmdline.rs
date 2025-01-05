use crate::libtkc::brightness::Brightness;
use crate::libtkc::light_pattern::LightPatternPublic;
use crate::libtkc::rgbcolor::ColorProfiles;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(
        short,
        long,
        value_enum)]
    pub brigthness: Option<Brightness>,
    #[arg(
        short,
        long,
        value_enum)]
    pub color: Option<ColorProfiles>,
    #[arg(
        short,
        long,
        value_enum)]
    pub pattern: Option<LightPatternPublic>,
}
