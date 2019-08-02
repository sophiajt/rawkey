use std::ptr;
use std::slice;
use x11::keysym;
use x11::xlib;

use crate::KeyCode;
//note: original logic from device_query
//TODO: non-X11 terminal input like showkey

pub struct RawKey {
    display: *mut xlib::Display,
}

impl RawKey {
    pub fn new() -> RawKey {
        unsafe {
            xlib::XInitThreads();
            let disp = xlib::XOpenDisplay(ptr::null());
            RawKey { display: disp }
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        match key {
            KeyCode::LeftArrow => self.query_keystate(keysym::XK_Left),
            KeyCode::RightArrow => self.query_keystate(keysym::XK_Right),
            KeyCode::UpArrow => self.query_keystate(keysym::XK_Up),
            KeyCode::DownArrow => self.query_keystate(keysym::XK_Down),
            KeyCode::LeftShift => self.query_keystate(keysym::XK_Shift_L),
            KeyCode::LeftControl => self.query_keystate(keysym::XK_Control_L),
            KeyCode::Escape => self.query_keystate(keysym::XK_Escape),
            KeyCode::Tab => self.query_keystate(keysym::XK_Tab),
            KeyCode::Space => self.query_keystate(keysym::XK_space),
            KeyCode::BackSpace => self.query_keystate(keysym::XK_BackSpace),
            KeyCode::PageDown => self.query_keystate(keysym::XK_Page_Down),
            KeyCode::PageUp => self.query_keystate(keysym::XK_Page_Up),
            KeyCode::Alt => self.query_keystate(keysym::XK_Alt_L),
        }
    }

    fn query_keystate(&self, key: u32) -> bool {
        unsafe {
            let keymap: *mut i8 = [0; 32].as_mut_ptr();
            xlib::XQueryKeymap(self.display, keymap);
            for (ix, byte) in slice::from_raw_parts(keymap, 32).iter().enumerate() {
                for bit in 0_u8..8_u8 {
                    let bitmask = 1 << bit;
                    if byte & bitmask != 0 {
                        let keycode = ix as u8 * 8 + bit;
                        let mut keysyms_per_keycode_return = 0_i32;
                        let keysym = xlib::XGetKeyboardMapping(
                            self.display,
                            keycode,
                            1,
                            &mut keysyms_per_keycode_return,
                        );

                        for ks in slice::from_raw_parts(keysym, keysyms_per_keycode_return as usize)
                            .iter()
                        {
                            if (*ks as u32) == key {
                                return true;
                            }
                        }

                        // Free the memory allocated by XGetKeyboardMapping.
                        xlib::XFree(keysym as *mut std::ffi::c_void);
                    }
                }
            }
            false
        }
    }
}
