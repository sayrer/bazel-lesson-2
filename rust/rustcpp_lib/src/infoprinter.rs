use libc::c_char;
use std::ops::Deref;
use std::ffi::CStr;

extern "C" {
    fn get_message() -> *const c_char;
    fn free_message(s: *const c_char);
}

pub struct InfoPrinter {
    message: *const c_char,
}

impl Drop for InfoPrinter {
    fn drop(&mut self) {
        unsafe {
            free_message(self.message);
        }
    }
}

impl InfoPrinter {
    pub fn new() -> InfoPrinter {
        InfoPrinter { message: unsafe { get_message() } }
    }
}

impl Deref for InfoPrinter {
    type Target = str;

    fn deref<'a>(&'a self) -> &'a str {
        let c_str = unsafe { CStr::from_ptr(self.message) };
        c_str.to_str().unwrap()
    }
}

