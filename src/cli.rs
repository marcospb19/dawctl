use clap::*;

pub(crate) fn parse_args() -> clap::ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .help_message("Display help information.")
        .version_message("Display version information.")
        .settings(&[AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("dpi")
                .long("--dpi")
                .short("-d")
                .value_name("DPI")
                .help("Sensor DPI (200 up to 6400)."),
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
                .help("Sensor frequency in Hz.")
                .possible_values(&["500", "1000"]),
        )
        .get_matches()
}
