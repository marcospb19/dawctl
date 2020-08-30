# dawctl
A device manager to control Razer's DeathAdder Essential White Edition mouse parameters in Linux (sensor DPI, frequency and white ligthing).

This manager was made by reverse engineering of the official razer synapse USBHID communication, using wireshark and exporting captures via .JSON, files used for analysis written in Python are in the folder reverse\_engineering/.

It's called dawctl because it is for the DAW (DeathAdder White).

The unsafe USBHID communication with crazy dark magic with bytes comes from https://github.com/9ary/da2013ctl, the CLI was also totally reworked.

Why da2013ctl won't work for the DAW? The devices are slighty different, for example, the DAW only has one option to control the brightness of both light spots (wheel and logo), this means that the chipset interface is different and bytes planned for da2013 won't work.

TODO:
- query subcommand!
- breathing ligthing effect!

### Installation

Please, keep in mind that I have yet to do the tests in different Linux distros, this SECTION IS A SCRATCH, and will change soon with update installation instructions.

```sh
cargo install --path .
```

Create the group `razer` and add yourself to it then:
```
# cargo install --root /usr/local
# sudo install -m644 50-da2013.rules /etc/udev/rules.d
# udevadm control --reload
# udevadm trigger
```

### Help
```sh
USAGE:
    dawctl [OPTIONS]

FLAGS:
    -h, --help       Display help information.
    -V, --version    Display version information.

OPTIONS:
    -l, --light <BRIGHTNESS_LEVEL>    Brightness level of the wheel and logo. [0-100]
    -d, --dpi <DPI>                   Sensor DPI (200 up to 6400).
    -f, --frequency <FREQUENCY>       Sensor frequency in Hz. [possible values: 500, 1000]
    -p, --path <PATH>                 Path to the hidraw node. (example: /dev/hidraw3)
```

### Examples
```sh
dawctl -l 50 # Set brightness
dawctl -d 3200 # Set dpi
dawctl --light 0 --dpi 600 --frequency 1000 # Brightness, dpi and frequency
dawctl --path /dev/hidraw2 -l 100 -d 1000 # Path too
```
