#![no_std]
#![no_main]
#![feature(abi_cherilibcall)]

use rtos_utils::cheriot_panic;

pub(crate) extern crate alloc;

/* Experiment 0: CHERIoT RTOS' allocator. */
pub(crate) mod cheriot_alloc;

/* Experiment 1: Import C functions. */
pub(crate) mod rtos_utils;

/* Experiment 2: Export Rust functions. */
pub mod arith;

/* Experiment 3: Rust's dynamic dispatch (+ FFI). */
pub(crate) mod zoo;

/* Experiment 4: CHERI permissions */
pub mod perms;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let str = alloc::string::ToString::to_string(&info.message());
    let str = <alloc::ffi::CString as core::str::FromStr>::from_str(&str).unwrap();
    unsafe {
        rtos_utils::cheriot_print_str(str.as_ptr());
        drop(str);
        cheriot_panic()
    };
    loop {}
}
