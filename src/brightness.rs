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
