use std::ffi::OsString;

use clap::Parser;

pub struct Args {
    pub dpi: Option<u16>,
    pub brightness: Option<u16>,
    pub frequency: Option<u16>,
    pub breath: bool,
    pub path: Option<OsString>,
}

impl Args {
    pub fn argparse() -> Self {
        ClapArgs::parse().into()
    }
}

impl From<ClapArgs> for Args {
    fn from(other: ClapArgs) -> Self {
        Self {
            dpi: other.dpi,
            brightness: other.brightness,
            frequency: other.frequency,
            breath: other.breath,
            path: other.path,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    arg_required_else_help = true,
    about = env!("CARGO_PKG_DESCRIPTION"),
    before_help = env!("CARGO_PKG_AUTHORS"),
    after_help = env!("CARGO_PKG_REPOSITORY"),
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct ClapArgs {
    /// Sensor DPI, multiples of 100. [200-6400]
    #[clap(short, long)]
    pub dpi: Option<u16>,

    /// Brightness level of the wheel and logo. [0-100]
    #[clap(short = 'l', long = "--light", value_name = "LEVEL")]
    pub brightness: Option<u16>,

    /// Sensor frequency in Hz. [500 or 1000]
    #[clap(short, long, possible_values = &["500", "1000"], hide_possible_values = true)]
    pub frequency: Option<u16>,

    /// Lighting breath effect.
    #[clap(short, long, conflicts_with = "brightness")]
    pub breath: bool,

    /// Path to the hidraw node. [example: /dev/hidraw3]
    #[clap(short, long)]
    pub path: Option<OsString>,

    /// Display help information.
    #[clap(short, long)]
    pub help: bool,

    /// Display version information.
    #[clap(short, long)]
    pub version: bool,
}
