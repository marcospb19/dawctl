# dawctl
Device manager to control Razer's DeathAdder Essential White Edition mouse **parameters** in _Linux_.

It's called dawctl because it is for the DAW (DeathAdder White edition).

Features:
- Automatic detection of the DAW.
- Flags:
  - --dpi: Sets the sensor DPI, from 200 up to 6400.
  - --light: Sets the brightness level of the light in the wheel and logo, from 0 up to 100.
  - --frequency: Changes the sensor frequency, 500 Hz or 1000 Hz.
  - --path: Path to the device, overwrites the automatic detection feature.

---

Lighting (0% and 35%):
<p float="left">
  <img src="https://user-images.githubusercontent.com/38900226/91668112-36221a80-eae0-11ea-8598-c03c9168812c.png" width="30%" />
  <img src="https://user-images.githubusercontent.com/38900226/91668105-27d3fe80-eae0-11ea-8bae-2cceb886cf92.png" width="30%" />
</p>

---

This manager was made by reverse engineering of the official razer synapse USBHID communication, using wireshark and exporting captures via .JSON, files used for analysis written in Python are in the folder reverse\_engineering/.


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

### Help (v0.1)
![help_image](https://user-images.githubusercontent.com/38900226/91664272-72e01880-eac4-11ea-8a41-8f03c463c520.png)

### Examples
```sh
dawctl -l 50
dawctl -d 3200
dawctl --light 0 --dpi 600 --frequency 1000
dawctl --path /dev/hidraw2 -l 100 -d 1000
```

---

I started this project because I really needed to control the DPI on my mouse, and turn this light off when up late in the night so I won't go blind.
