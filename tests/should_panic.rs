#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rs_os::{QemuExitCode, exit_qemu, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    shuold_fail();
    serial_println!("[test did not fail]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn shuold_fail() {
    serial_println!("should fail...");
    assert_eq!(0, 1);
}