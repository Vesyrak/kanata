// uinput-rs
use uinput_sys;
use uinput_sys::uinput_user_dev;

use evdev_rs::InputEvent;
use libc::input_event as raw_event;

// file i/o
use std::fs::OpenOptions;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::io;
use io::Write;

// unsafe
use std::slice;
use std::mem;

pub struct KbdOut {
    device: File,
}

impl KbdOut {
    pub fn new() -> Result<Self, io::Error> {
        let mut uinput_out_file = OpenOptions::new().read(true).write(true).open("/dev/uinput")?;

        unsafe {
            uinput_sys::ui_set_evbit(uinput_out_file.as_raw_fd(), uinput_sys::EV_SYN);
            uinput_sys::ui_set_evbit(uinput_out_file.as_raw_fd(), uinput_sys::EV_KEY);

            for key in 0..uinput_sys::KEY_MAX {
                uinput_sys::ui_set_keybit(uinput_out_file.as_raw_fd(), key);
            }

            let mut uidev: uinput_user_dev = mem::zeroed();
            uidev.name[0] = 'k' as i8;
            uidev.name[1] = 't' as i8;
            uidev.name[2] = 'r' as i8;
            uidev.name[3] = 'l' as i8;
            uidev.id.bustype = 0x3; // BUS_USB
            uidev.id.vendor  = 0x1;
            uidev.id.product = 0x1;
            uidev.id.version = 1;

            let uidev_bytes = slice::from_raw_parts(mem::transmute(&uidev),
                                                    mem::size_of::<uinput_user_dev>());
            uinput_out_file.write(uidev_bytes)?;
            uinput_sys::ui_dev_create(uinput_out_file.as_raw_fd());
        }

        Ok(KbdOut{device: uinput_out_file})
    }

    pub fn write(&mut self, event: &InputEvent) -> Result<(), io::Error> {
        let ev = event.as_raw();

        unsafe {
            let ev_bytes = slice::from_raw_parts(mem::transmute(&ev as *const raw_event),
                                                 mem::size_of::<raw_event>());
            self.device.write(ev_bytes)?;
        };

        Ok(())
    }
}
