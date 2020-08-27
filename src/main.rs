#[macro_use]
extern crate nix;

mod cli;
mod daw;
mod usbhid_communication;

use std::process;

// Ids to search when automatically finding plugged DeathAdderWhite
// Change this fields if necessary
const ID_VENDOR: &str = "1532";
const ID_PRODUCT: &str = "0071";

fn main() {
    // clap usage, cli.rs
    // Grabs Option<&str> for each flag
    let flags = cli::parse_args();

    let path_flag = flags.value_of("path");
    // Tries to detect mouse device using ID_VENDOR and ID_PRODUCT.
    // The path can be overwritten by --path flag!
    let mouse = daw::DeathAdderWhite::new(path_flag, ID_VENDOR, ID_PRODUCT).unwrap_or_else(|err| {
        eprintln!("Unexpected read error: {}.", err);
        process::exit(1);
    });

    if let Some(level) = flags.value_of("brightness") {
        mouse.set_brightness(level);
    }
    if let Some(dpi) = flags.value_of("dpi") {
        mouse.set_dpi(dpi);
    }
    if let Some(frequency) = flags.value_of("frequency") {
        mouse.set_frequency(frequency);
    }
}
