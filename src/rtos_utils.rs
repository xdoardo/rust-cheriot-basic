//! Utility functions imported from external sources (RTOS tests, ...)

unsafe extern "C" {
    pub fn cheriot_print_str(v: *const core::ffi::c_char);
    pub fn cheriot_alloc(bytes: u32) -> *mut core::ffi::c_void;
    pub fn cheriot_free(ptr: *mut core::ffi::c_void);
    pub fn cheriot_panic();
    pub fn cheriot_random_byte() -> u8;
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::rtos_utils::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    let str = alloc::string::ToString::to_string(&args);
    let str = alloc::ffi::CString::new(str).unwrap();

    unsafe {
        cheriot_print_str(str.as_ptr());
    }

    drop(str);
}
