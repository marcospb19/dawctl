mod cli;
mod daw;
mod error;
mod usbhid_communication;

use daw::DeathAdderWhite;

fn main() {
    let args = cli::Args::argparse();

    let mouse = DeathAdderWhite::new(args.path).unwrap_or_else(|err| {
        runtime_error!("Unexpected read error: {}.", err);
    });

    if let Some(level) = args.brightness {
        mouse.set_brightness(level);
    }
    if let Some(dpi) = args.dpi {
        mouse.set_dpi(dpi);
    }
    if let Some(frequency) = args.frequency {
        mouse.set_frequency(frequency);
    }
    if args.breath {
        mouse.run_breath_effect();
    }
}
