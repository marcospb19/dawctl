use clap::Parser;
use std::ffi::OsString;

impl Args {
    pub fn argparse() -> Self {
        Self::parse()
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    /// Sensor DPI, multiples of 100. [200-6400].
    #[clap(short, long, value_name = "DPI")]
    pub dpi: Option<u16>,

    /// Brightness level of the wheel and logo. [0-100]
    #[clap(short = 'l', long = "--light", value_name = "BRIGHTNESS_LEVEL")]
    pub brightness: Option<u16>,

    /// Sensor frequency in Hz. [500 or 1000]
    #[clap(short, long, value_name = "FREQUENCY", possible_values = &["500", "1000"], hide_possible_values = true)]
    pub frequency: Option<u16>,

    /// Lighting breath effect.
    #[clap(short, long)]
    pub breath: bool,

    /// Path to the hidraw node. (example: /dev/hidraw3).
    #[clap(short, long, value_name = "PATH")]
    pub path: Option<OsString>,
}
