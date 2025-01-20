#![no_std] // do not link to Rust std lib
#![no_main] 
#![feature(custom_test_frameworks)]
#![test_runner(rs_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rs_os::println;

#[no_mangle] // do not mangle fn name
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "Rust !");

    #[cfg(test)]
    test_main();
    loop{}
}
// This function will be called when panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rs_os::test_panic_handler(info)
}