# Dawctl
Super simple, minimal, and fast CLI device manager to control Razer's Deathadder Essential White Edition mouse **parameters** in _Linux_.

The software itself is 100% written in Rust, Python was used only for data analysis in the reverse engineering process (more about it at the end of this file).

It's called dawctl because it is a controller (ctl) for the Deathadder White edition (DAW, for short).

Features:
- Automatic detection of the DAW, if plugged in ;).
- Flags:
  - `-d --dpi`: Change the sensor DPI, from 200 up to 6400.
  - `-l --light`: Set level of brightness of the lights in the wheel and logo, from 0 up to 100.
  - `-f --frequency`: Change sensor frequency, 500 Hz or 1000 Hz.
  - `-p --path`: Path to the device, overwrites the automatic detection feature.

I'm proud to say that our `--light` parameter allows for better ranges than Razer Synapse's one! For the lower values you have more options to choose :D (if you're like batman and you stay up until absolute darkness, you'll love this).

---

Lighting (0%, 7% and 70%):
<p float="left">
  <img src="https://i.imgur.com/8XviPEf.jpg" width="31%" />
  <img src="https://i.imgur.com/MkZTpcB.jpg" width="31%" />
  <img src="https://i.imgur.com/Arrm9SC.jpg" width="31%" />
</p>

TODO:
- Query subcommand! (but get information about current parameters).
- Breathing ligthing effect! (and others, like blinking in specific intervals, X times).
- Updating `nix` version to latest (ioctl changes).
- Test on a big-endian machine.

### Installation
Note 1: this section does not cover how to compile `dawctl`, as it's the same procedure for every Rust project.
Note 2: libc is a dependency, but it's probably installed in your machine ('libc.so.6').

- Steps
  - Step 1: Grab a binary from the `releases section` (around 800 KB) on github and download it.
  - Step 2: Move it to `/usr/bin/dawctl` to install it in the system (requires sudo).

Done! Well... almost. Now the script is available for all users, but Linux won't give permission for any user to communicate freely with the hardware, you would need to type `sudo` every time, however, you can create an exception by adding a "rule", here's how it works:

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

### simplified list of commands
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
dawctl -l 50
dawctl --light 50
dawctl -d 700
dawctl --dpi 700
dawctl -f 1000
dawctl --frequency 1000
```
---

## Extra info about this project:
### How was this created?

I started this project because I really needed to control the DPI on my mouse, and I wanted to turn off this lights so I won't go blind.

This manager was made by reverse engineering of the official razer synapse USBHID communication, using wireshark and exporting captures via .JSON, then using the files for analysis written in Python, scripts are in the folder reverse\_engineering/.

The hardest part of this project (in the view of someone that knows nothing about USBHID raw communication in Linux), comes from https://github.com/9ary/da2013ctl, without `9ary`, I wouldn't be able to lie on top of his abstraction and spend my time in the reverse engineering process.

In case you're confused, `9ary` did a great job with `da2013ctl`, but it won't work with the DAW because the device interface and the hardware features are different from those versions.

### Why isn't it in crates.io?
Sadly, it is currently [not possible](https://github.com/dcuddeback/libudev-rs/pull/10#issuecomment-683534098) because I need `libudev` from a specific commit because the API wasn't correctly exposed.

### What about razercfg?
I seeker in source files and I think that it does not supports the DAW, but honestly, I'm not even capable of making that run in my machine.

---

## Helping
If you like my project, thank!, it is very easy to help me, here are some options:

1. Giving a star and sharing this software with someone that might be interested in.
2. Help me improve this README (I would really appreciate it).
3. You can create an issue here for any reason, even if you just want to ask a question.
4. Helping with code, debugging, or reporting errors.
