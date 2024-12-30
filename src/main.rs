#![no_std] // do not link to Rust std lib
#![no_main] 

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // do not mangle fn name
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}
// This function will be called when panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}