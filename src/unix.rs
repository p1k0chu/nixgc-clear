use crate::error::Error;
use std::ffi::{CStr, c_char, c_int};
use std::{iter, ptr};

unsafe extern "C" {
    fn execvp(path: *const c_char, argv: *const *const c_char) -> i32;
    fn __errno_location() -> *mut c_int;
    pub safe fn geteuid() -> u32;
    pub safe fn getegid() -> u32;
    fn strerror(errno: c_int) -> *const c_char;
}

pub fn execvp_safe(path: &CStr, argv: &[&CStr]) -> Error {
    let argv: Vec<_> = iter::once(&path)
        .chain(argv.iter())
        .map(|i| (*i).as_ptr())
        .chain(iter::once(ptr::null()))
        .collect();

    let msg = unsafe {
        execvp(path.as_ptr(), argv.as_ptr());
        let errno = *__errno_location();
        let msg = strerror(errno);
        CStr::from_ptr(msg)
    };

    format!("execvp failed: {}", msg.to_string_lossy()).into()
}
