use std::ffi::{c_char, CString};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("main_lib.h");
        unsafe fn _main(argc: i32, argv: *mut *mut c_char) -> i32;
    }

    extern "Rust" {
        fn hello_rust();
    }
}


pub fn hello_rust() {
    println!("Hello, Rust!");
}

pub fn cpp_main() {
    let args = std::env::args().collect::<Vec<String>>();
    cpp_main_with_args(args)
}
pub fn cpp_main_with_args<T: AsRef<str>>(args: Vec<T>) {
    let cstr_args = args
        .iter()
        .map(|s| CString::new(s.as_ref()).unwrap())
        .collect::<Vec<_>>();
    let mut cstr_pointers: Vec<*mut c_char> = cstr_args
        .iter()
        .map(|s| s.as_ptr() as *mut c_char)
        .collect::<Vec<_>>();
    cstr_pointers.push(std::ptr::null_mut());
    unsafe {
        ffi::_main(cstr_pointers.len() as i32 - 1, cstr_pointers.as_mut_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        let args = vec!["test"];
        cpp_main_with_args(args);
    }
}
