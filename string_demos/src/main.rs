use std::{ffi::{CStr, CString}, os::raw::c_char};

fn main() {
    let _s = CString::new("abcdef");
}

#[no_mangle]
pub fn receive_str_and_return_string(s: *const c_char) -> *const c_char {
    let cstr = {
        assert!(!s.is_null());
        unsafe { CStr::from_ptr(s) }
    };
    let rstr = cstr.to_str().expect("not valid untf-8 string");

    todo!()
}
