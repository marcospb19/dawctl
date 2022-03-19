use std::{ffi::OsStr, io, os::unix, path::PathBuf, thread::sleep, time::Duration};

use nix::{fcntl, sys::stat::Mode, unistd};

use crate::{runtime_error, usbhid_communication};

pub struct DeathAdderWhite {
    file_descriptor: unix::io::RawFd,
}

// Ids to search when automatically finding plugged DeathAdderWhite
// Change this fields if necessary
const ID_VENDOR: &str = "1532";
const ID_PRODUCT: &str = "0071";

impl DeathAdderWhite {
    // Return instance of DeathAdderWhite
    //
    // Tries to detect mouse device by ID_VENDOR and ID_PRODUCT using libudev
    // functions.
    //
    // But the path can be overwritten by --path flag.
    pub fn new<O>(path_flag: Option<O>) -> io::Result<Self>
    where
        O: AsRef<OsStr>,
    {
        let udev_context = &libudev::Context::new()?;
        let mut enumerator = libudev::Enumerator::new(udev_context)?;

        // Apply a filter for the enumerator
        enumerator.match_subsystem("hidraw")?;

        let mouse_device_path = if let Some(path) = path_flag {
            PathBuf::from(path.as_ref())
        } else {
            let mut option_path: Option<PathBuf> = None;
            for device in enumerator.scan_devices()? {
                if let Some(usb_parent) = device.parent_with_subsystem_devtype("usb", "usb_device") {
                    let device_id_vendor = usb_parent
                        .attribute_value("idVendor")
                        .expect("Error: unable to read the device idVendor.");
                    let device_id_product = usb_parent
                        .attribute_value("idProduct")
                        .expect("Error: unable to read the device idProduct.");
                    if device_id_vendor == ID_VENDOR && device_id_product == ID_PRODUCT {
                        // println!("Found!");
                        option_path = Some(PathBuf::from(device.devnode().unwrap()));
                        break;
                    }
                }
            }
            if let Some(found_path) = option_path {
                found_path
            } else {
                runtime_error!("Error: dawctl was unable to detect the device hidraw node with udev, make sure it's \
                          plugged in.\n\
                          You can also specify it's path using the flag --path, install.\n\
                          \n\
                          For debugging on this problem, install `usbutils` and run `lsusb`, for the DeathAdder White\
                          Edition you're looking for the code 1532:0071, this means productId 1532 and vendorId 0071,\
                          this value may vary for different mice devices. If `lsusb` don't list Razer DeathAdder, it\
                          means it's not connected, if this appears but with different code, please open an issue on\
                          the problem (https://github.com/marcospb19/dawctl).");
            }
        };

        let file_descriptor =
            fcntl::open(&mouse_device_path, nix::fcntl::O_RDWR, Mode::empty()).unwrap_or_else(|err| {
                runtime_error!("Failed to open device: {}", err.to_string());
            });

        Ok(DeathAdderWhite { file_descriptor })
    }

    pub fn set_dpi(&self, dpi: u16) {
        if !(200..=6400).contains(&dpi) {
            runtime_error!("Error: DPI_LEVEL isn't in the valid interval [200-6400]: '{}'", dpi);
        }

        if dpi % 100 != 0 {
            runtime_error!("Error: DPI_LEVEL isn't a multiple of 100: '{}'.", dpi);
        }

        let cmd: Vec<u8> = vec![0x07, 0x04, 0x05, 0x01];
        let mut args = dpi.to_be_bytes().to_vec();
        args.extend(args.clone()); // Duplicate it's size
        let footer = 0x07;
        self.send_cmd(cmd, args, footer);
    }

    pub fn set_frequency(&self, frequency_flag: u16) {
        // Sequence of bytes to
        let cmd: Vec<u8> = vec![0x07, 0x04, 0x05, 0x01];
        // Bytes for frequency argument and each sequence footer
        let (frequency, footer) = match frequency_flag {
            500 => (0x02, 0x06),
            1000 => (0x01, 0x05),
            _ => unreachable!(),
        };
        self.send_cmd(cmd, vec![frequency], footer);
    }

    pub fn run_breath_effect(&self) {
        loop {
            for i in (0..100).chain((0..100).rev()) {
                sleep(Duration::from_millis(30));
                self.set_brightness(i);
            }
            sleep(Duration::from_millis(30));
        }
    }

    pub fn set_brightness(&self, brightness: u16) {
        if brightness > 100 {
            runtime_error!("Error: Brightness level '{}' is out of the range [0,100].", brightness);
        }

        // Command sequence to change brightness
        let cmd: Vec<u8> = vec![0x03, 0x0f, 0x04, 0x01, 0x00];

        // Confusing math ahead!!!!!!!
        // Please, read the comments to understand it.
        //
        // We receive the brightness_level in the range [0-100], but the device receives
        // it in the range [0-255], so we need to translate it, this means that we'll
        // have gaps, the synapse code just skips numbers in a step of size 2.55 and
        // assumes everything is just fine.
        //
        // We, however will be more thoughtful of our actions
        //
        // If you mess with the light values, you'll notice a very large difference
        // between low values and little to no difference from [50-100], with this in
        // mind, we'll make a transformation using an exponential function to make a
        // translation in the following way:
        //
        // The start of the sequence grows as slowly as possible (1 by 1) and then the
        // sequence starts to slowly speed up it's growth.
        //
        // This way, we give the user a chance to choose with precision low values where
        // if the level changes by 1, the difference is noticeable, in exchange of the
        // precision at the end of the sequence (closer to 255), where the level changes
        // makes almost no visible difference.
        //
        //
        // Before: [0, 1, 2, 3, 4 ... 57, 58, 59,  60 ...  96,  97,  98,  99, 100]
        // After:  [0, 1, 2, 3, 4 ... 92, 94, 97, 100 ... 235, 240, 245, 250, 255]
        //
        //                                 ↓ Here, about 2.7 growth rate
        // Growth: [_, 1, 1, 1, 1 ...  3,  2,  3,   3 ...   5,   5,   5,   5,   5]
        //
        // The exp function, by default, will generate a sequence that starts fast and
        // slows at the end, because we want the other way around, se let's start with
        // `100 - level` to reverse it to fit our need
        let level = 100 - brightness;

        // Scale it down, value is between 0.00 and 1.00
        let level: f64 = level as f64 / 100.0;

        // Exponential function that eases stuff. And the multiplication forces the
        // sequence end to converge exactly to 0, now the range ends are really aligned
        let level: f64 = (level * -1.07f64).exp() * (-level + 1.0);

        // Now we can scale it up again, and round the value. The rounded value will go
        // exactly to the bits we wanted, this was carefully picked up.
        // One more hidden detail, the true sequence starts at 2 instead of 1, so we'll
        // use 254 and add 1, instead of using 255
        let level: u8 = (level * 254.0).round() as u8 + 1;

        // Get new value and move on :D
        let brightness = level;
        // println!("{}", brightness);

        let footer = brightness.to_le_bytes()[0] ^ 0x09;
        self.send_cmd(cmd, vec![brightness], footer);
    }

    fn send_cmd(&self, cmd: Vec<u8>, args: Vec<u8>, footer: u8) {
        // Buffer used to communicate
        let mut buf = Vec::with_capacity(256);

        // 1 // HID report number
        // + 1 // Status
        // + 4 // Padding
        // = 6
        let zeros = vec![0x00; 6];
        buf.extend(zeros);
        for cmd_byte in &cmd {
            buf.push(cmd_byte.to_le_bytes()[0]);
        }
        for arg_byte in &args {
            buf.push(arg_byte.to_le_bytes()[0]);
        }
        let zeros = vec![0x00; 89 - buf.len()];
        buf.extend(zeros);
        buf.push(footer.to_le_bytes()[0]);
        buf.push(0x00);

        // Try 4 times to communicate with the device successfully.
        // If no success, exit.
        for _ in 0..4 {
            unsafe {
                // Send command to DeathAdder, skip if error
                if let Err(err) = usbhid_communication::sfeature(self.file_descriptor, buf.as_mut_ptr(), buf.len()) {
                    eprintln!("USBHID_IOCSFEATURE: {}", err);
                    eprintln!("error, trying again maybe this time it'll work shit");
                    continue;
                }

                // Communication: receive response from mouse
                // We are overwriting the same buffer used for sending the message
                if let Err(err) = usbhid_communication::gfeature(self.file_descriptor, buf.as_mut_ptr(), buf.len()) {
                    eprintln!("USBHID_IOCGFEATURE: {}", err);
                    continue;
                }
            }
            match buf[1] {
                // Check if device responded successfully
                // We expect the same values as librazer/razercfg does
                0..=3 => {
                    // println!("DONE!");
                    return;
                }
                other => eprintln!(
                    "Command failed: Device did not answered what we expected: '{}'.\ncmd_bytes: {:#?}",
                    other, cmd
                ),
            }
        }
    }
}

impl Drop for DeathAdderWhite {
    fn drop(&mut self) {
        unistd::close(self.file_descriptor).expect("Failed to close device");
    }
}
