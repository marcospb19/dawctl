use clap::*;

pub(crate) fn parse_args() -> clap::ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .help_message("Display help information.")
        .after_help("Please, contribute and leave issues at https://github.com/marcospb19/dawctl")
        .version_message("Display version information.")
        .settings(&[AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("dpi")
                .long("--dpi")
                .short("-d")
                .value_name("DPI")
                .help("Sensor DPI, multiples of 100. [200-6400]"),
        )
        .arg(
            Arg::with_name("path")
                .long("--path")
                .short("-p")
                .value_name("PATH")
                .help("Path to the hidraw node. (example: /dev/hidraw3)"),
        )
        .arg(
            Arg::with_name("brightness")
                .long("--light")
                .short("-l")
                .value_name("BRIGHTNESS_LEVEL")
                .help("Brightness level of the wheel and logo. [0-100]"),
        )
        .arg(
            Arg::with_name("frequency")
                .long("--frequency")
                .short("-f")
                .value_name("FREQUENCY")
                .help("Sensor frequency in Hz. [500 or 1000]")
                .possible_values(&["500", "1000"])
                .hide_possible_values(true),
        )
        .arg(
            Arg::with_name("breath")
                .long("--breath")
                .short("-b")
                .help("Lighting breath effect."),
        )
        .get_matches()
}
