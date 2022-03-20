# Dawctl
Super simple and instant CLI device manager to control Razer's Deathadder Essential White Edition mouse **parameters** in _Linux_.

Written in Rust, used Python for data analysis while reverse engineering proprietary Razer software.

## Features:
- Automatic detection of the DAW, if plugged in ;).
- Flags:
  - `-d --dpi`: Change the sensor DPI, from 200 up to 6400.
  - `-l --light`: Set level of brightness of the lights in the wheel and logo, from 0 up to 100.
  - `-f --frequency`: Change sensor frequency, 500 Hz or 1000 Hz.
  - `-p --path`: Path to the device, overwrites the automatic detection feature.

By reverse engineering light values, I found out that the default presetted slider in Razer Synapse is limited and skips values, I have made some math magic and now we can achieve lower and more detailed levels of luminosity.

This is very convenient to people that stay up until it's very late, cause now you can set the lower values without going blind!

It was worth it.

---

Lighting (0%, 7% and 70%):
<p float="left">
  <img src="https://i.imgur.com/8XviPEf.jpg" width="31%" />
  <img src="https://i.imgur.com/MkZTpcB.jpg" width="31%" />
  <img src="https://i.imgur.com/Arrm9SC.jpg" width="31%" />
</p>

## Unimplemented:
- Query subcommand to get information about current mouse parameters, instead of just overwritting them.
- Breathing ligthing effect, it's not hard to implement, but I don't feel like I really need it xD.
- Updating `nix` version to latest (ioctl changes).
- Test on a big-endian machine.

## Installation from pre-built binary
Note: `libc` is a dependency but should be already installed.

 - Step 1: Grab a binary from the `releases section` (around 800 KB) on github and download it.
 - Step 2: Move it to `/usr/bin/dawctl` to install it in the system (requires sudo).

If you want to run it without sudo, [see this](https://github.com/marcospb19/dawctl/wiki/Running-without-sudo).

## Dependencies for building

- [libudev](https://github.com/dcuddeback/libudev-sys)

### Help (v0.3)
![help_image](https://user-images.githubusercontent.com/38900226/159143362-dccdea52-14f8-434b-929a-b916518546b0.png)


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
### Why was this created?

I started this project because I needed to change the crazy DPI value of my mouse, and I wanted to turn off the lights so I won't go blind.

This manager was made by reverse engineering of the official razer synapse USBHID communication, using wireshark and exporting captures via `.JSON`, then displaying the bytes using Python, so then I had to understand what those byte packages meant, I already had a hint from [@9ary](https://github.com/9ary) that some information on the packages go on the top, and there's a footer byte at the end.

Python scripts for printing 1s and 0s are in the folder `reverse\_engineering/`.

There's still one thing that I don't understand, and I expected to not understand it cause I never knew that USBHID existed (to me USBs just worked because YES), and this thing is a dark magic macro by https://github.com/9ary/da2013ctl that actually sends the packages, using the `nix` library, so ty [@9ary](https://github.com/9ary) for letting me be able to lie on top of your abstraction and focus my time in the reverse engineering process, he's some sort of USB protocols expert so check him out and hire him lol.

This is the project that I've had most fun working on.

### About alternatives
There are some huge Razer Synapse reverse engineer C projects (like [`razercfg`](https://github.com/mbuesch/razer)) that already had support for the `daw`, but I found it so confusing to use that I thought it was better to create my own.

### Why isn't it in crates.io?
It is currently not possible, because we use a dependency from a specific git tree that is non-published.

---

## Helping
If you like my project, it is very easy to help me, here are some options:

1. Giving a star and sharing this software with someone that might be interested in.
2. Help me improve this README (I would really appreciate it).
3. You can create an issue here for any reason, even if you just want to ask a question.
4. Helping with code, debugging, or reporting errors.

Thanks!
