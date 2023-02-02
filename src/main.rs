#![no_std]
#![no_main]

use core::panic::PanicInfo;


// Overwriting the entry , it will be called by OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// Its called when a panic occours
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}