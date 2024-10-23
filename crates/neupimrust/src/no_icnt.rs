use std::ffi::c_void;

#[repr(C)]
pub struct NoIcnt {
    total_packages: usize,
}

impl NoIcnt {
    pub fn new() -> Self {
        NoIcnt { total_packages: 0 }
    }

    #[no_mangle]
    pub extern "C" fn get_total_packages(&self) -> usize {
        self.total_packages
    }

    #[no_mangle]
    pub extern "C" fn push(&mut self, src: u32, dest: u32, request: *const c_void) {
        let _ = request;
        let _ = dest;
        let _ = src;
    }
}

#[no_mangle]
pub extern "C" fn new_icnt() -> *mut NoIcnt {
    Box::into_raw(Box::new(NoIcnt::new()))
}

#[no_mangle]
pub extern "C" fn delete_icnt(ptr: *mut NoIcnt) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
