use crate::error::Error;
use std::ffi::{CStr, c_char, c_int};
use std::iter;

unsafe extern "C" {
    fn execvp(path: *const c_char, argv: *const *const c_char) -> i32;
    fn __errno_location() -> *mut c_int;
    pub safe fn geteuid() -> u32;
    pub safe fn getegid() -> u32;
}

pub fn execvp_safe(path: &CStr, argv: &[&CStr]) -> Error {
    let argv: Vec<_> = iter::once(path)
        .chain(argv.to_owned().into_iter())
        .map(|i| i.as_ptr())
        .collect();

    let c = unsafe {
        execvp(path.as_ptr(), argv.as_ptr());
        *__errno_location()
    };
    format!("execvp failed: {}", c).into()
}
