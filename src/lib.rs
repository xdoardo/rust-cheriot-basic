#![no_std]
#![no_main]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn div(a: u64, b: u64) -> u64 {
    a / b
}

#[unsafe(no_mangle)]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn zero() -> u32 {
    0
}
