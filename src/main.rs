#![no_std]
#![no_main]

use core::panic::PanicInfo;


/// Its called when a panic occours
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


// Overwriting the entry , it will be called by OS
// _start() call, are the same of Linux entry call (just for reference)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}