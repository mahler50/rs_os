#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}
// This function will be called when panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}