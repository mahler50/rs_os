#![no_std] // do not link to Rust std lib
#![no_main] 

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] // do not mangle fn name
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "Rust !");
    panic!("Some panic message");
    loop{}
}
// This function will be called when panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}