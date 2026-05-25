use std::ffi::CString;
use std::os::raw::c_char;

unsafe extern "C" {
    fn reverse_string(s: *mut c_char, l: usize) -> c_char;
}

fn main() {
    let rust_string = String::from("Rust FFI Example in Chapter 4");
    let mut c_string = CString::new(rust_string).expect("CString::new failed!");

    let length = c_string.as_bytes().len();
    let c_ptr = c_string.into_raw();

    unsafe { reverse_string(c_ptr, length) };

    let reversed_word = unsafe {
        CString::from_raw(c_ptr)
            .into_string()
            .expect("CString::from_raw failed!")
    };
    println!("Reversed Word: {}", reversed_word)
}
