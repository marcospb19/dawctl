# dawctl
Super simple (and fast) CLI device manager to control Razer's Deathadder Essential White Edition mouse **parameters** in _Linux_.

The software itself is 100% written in Rust, Python is used only for external analysis of the reverse engineering process (read more at the end of this file).

It's called dawctl because it is for the DAW (Deathadder White edition).

Features:
- Automatic detection of the DAW.
- Flags:
  - --dpi: Change the sensor DPI, from 200 up to 6400.
  - --light: Set level of brightness of the lights in the wheel and logo, from 0 up to 100.
  - --frequency: Change sensor frequency, 500 Hz or 1000 Hz.
  - --path: Path to the device, overwrites the automatic detection feature.

---

Lighting (0% and 56%):
<p float="left">
  <img src="https://user-images.githubusercontent.com/38900226/91668112-36221a80-eae0-11ea-8598-c03c9168812c.png" width="30%" />
  <img src="https://user-images.githubusercontent.com/38900226/91668105-27d3fe80-eae0-11ea-8bae-2cceb886cf92.png" width="30%" />
</p>

TODO:
- query subcommand!
- breathing ligthing effect!

### Installation
Note 1: this section does not cover how to compile `dawctl`, as it's the same procedure for every Rust project.
Note 2: libc is a dependency, but it's probably installed in your machine.
Note 3: if you want to skip the explanation, at the end of the section there's a simplified list of commands.

- Steps
  - Step 1: Grab a binary from the `releases section` (around 800 KB) on github and download it.
  - Step 2: Move it to `/usr/bin/dawctl` to install it in the system (requires sudo).

Done! Well... almost. Now the script is available for all users, but Linux won't give permission for any user to communicate with the hardware, you would need to type `sudo` every time, however, you can create an exception by adding a "rule", here's how it works:

Create a file, at `/etc/udev/rules.d/99-hidraw-permissions.rules`, and copy this:
```py
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0664", GROUP="wheel"
```

This adds the permission to all users in the group `wheel`, by default (in many distributions) your personal user is already included! Users that can type `sudo` are inside of the `wheel` group, `hidraw` is the Linux way to enable usbHID RAW communication.

Now, you need to update kernel events:
```sh
sudo udevadm control --reload
sudo udevadm trigger
```

#### simplified list of commands
After you installed the binary from https://github.com/marcospb19/dawctl/releases/latest at /usr/bin/dawctl.
```sh
sudo -c 'echo KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0664", GROUP="wheel"' > /etc/udev/rules.d/99-hidraw-permissions.rules
sudo udevadm control --reload
sudo udevadm trigger
dawctl --help
```

### Help (v0.1)
![help_image](https://user-images.githubusercontent.com/38900226/91664272-72e01880-eac4-11ea-8a41-8f03c463c520.png)

### Examples
```sh
dawctl -l 50 #
dawctl -d 3200
dawctl --light 0 --dpi 600 --frequency 1000
dawctl --path /dev/hidraw2 -l 100 -d 1000
```

---

## Extra info about this project:
## How was this created?

I started this project because I really needed to control the DPI on my mouse, and turn this light off when up late in the night so I won't go blind.

This manager was made by reverse engineering of the official razer synapse USBHID communication, using wireshark and exporting captures via .JSON, files used for analysis written in Python are in the folder reverse\_engineering/.

The unsafe USBHID communication with crazy dark magic with bytes comes from https://github.com/9ary/da2013ctl, the CLI was totally reworked.

Sadly, da2013ctl (link above) won't work for the DAW because the device interface and the hardware features are different from those versions.
