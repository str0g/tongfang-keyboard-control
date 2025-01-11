use crate::libtkc::brightness::Brightness;
use crate::libtkc::light_pattern::LightPatternPublic;
use crate::libtkc::rgbcolor::ColorProfiles;

use clap::Parser;
#[cfg(feature = "completions")]
use clap::builder::{BoolishValueParser, TypedValueParser};

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

#[cfg(feature = "completions")]
    #[arg(
        long,
        required(false),
        exclusive(true),
        action(clap::ArgAction::SetTrue),
        value_parser(BoolishValueParser::new().try_map(completions::generate_completions)))]
    pub completions: bool,
}

#[cfg(feature = "completions")]
mod completions {
use clap_complete::{generate_to, shells::*};
use clap::CommandFactory;
use std::env;
use std::io::Error;
use std::fs;
use crate::cmdline::Args;

pub fn generate_completions(ignore: bool) -> Result<bool, Error> {
    //let outdir = match env::var_os("OUT_DIR") {
    //    None => panic!("Env variable does not exist or its empty"),
    //    Some(outdir) => std::path::Path::new(&outdir).join("completions/"),
    //};
    if ignore {
        let outdir = match env::current_exe() {
            Ok(outdir) => std::path::Path::new(outdir.parent().expect("does not have parent?")).join("completions/"),
            Err(e) => panic!("Current exec {:?}",e ),
        };

        if !fs::exists(&outdir).unwrap() {
            fs::create_dir(&outdir)?;
        }
        generate_to(Bash, &mut Args::command(), "tongfang-keyboard-control", &outdir).expect("bash");
        generate_to(Zsh, &mut Args::command(), "tongfang-keyboard-control", &outdir).expect("zsh");
        generate_to(Fish, &mut Args::command(), "tongfang-keyboard-control", &outdir).expect("fish");
    }
    Ok(ignore)
}
}
