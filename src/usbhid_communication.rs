const USBHID_IOC_MAGIC: u8 = b'H';
const USBHID_IOC_NR_SFEATURE: u8 = 0x06;
const USBHID_IOC_NR_GFEATURE: u8 = 0x07;

use nix::*;

// This macro implements the function used to SEND informations to the device
nix::ioctl!(readwrite buf sfeature with USBHID_IOC_MAGIC, USBHID_IOC_NR_SFEATURE; u8);

// This macro implements the function used to GET informations to the device
nix::ioctl!(readwrite buf gfeature with USBHID_IOC_MAGIC, USBHID_IOC_NR_GFEATURE; u8);
