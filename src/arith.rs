use crate::println;

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

#[unsafe(no_mangle)]
pub extern "C" fn arith_tour() {
    println!("zero: {}", zero());

    println!("add(5, 5): {}", add(5, 5));

    println!("div(8, 4): {}", div(8, 4));
}
