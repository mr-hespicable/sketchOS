#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sketch_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_breakpoint();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sketch_os::test_panic_handler(info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn test_breakpoint() {
    serial_print!("breakpoint_interrupt::should_breakpoint_error...\t");
    x86_64::instructions::interrupts::int3();
}
