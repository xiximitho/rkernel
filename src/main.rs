#![no_std]
#![no_main]

use core::panic::PanicInfo;


/// Its called when a panic occours
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



static HELLO: &[u8] = b"Um simples kernel";

// Overwriting the entry , it will be called by OS
// _start() call, are the same of Linux entry call (just for reference)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}