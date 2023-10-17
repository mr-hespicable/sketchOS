#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sketch_os::{exit_qemu, serial_println, QemuExitCode};

static mut SHOULD_FAIL: bool = false;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    sketch_os::init(); //init idt
    test_main();
    #[allow(clippy::empty_loop)]
    loop {}
}

fn should_fail() -> bool {
    //if this is true: fail panic thing, else: non-fail panic thing.
    unsafe { SHOULD_FAIL }
}

fn set_should_fail(val: bool) {
    unsafe {
        SHOULD_FAIL = val;
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if should_fail() {
        panic_should_fail(info);
    } else {
        sketch_os::test_panic_handler(info)
    }
}

fn panic_should_fail(_info: &PanicInfo) -> ! {
    set_should_fail(false);
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

//tests go here
#[test_case]
fn test_breakpoint() {
    x86_64::instructions::interrupts::int3();
}

//double fault will go here
#[test_case]
fn test_double_fault() {
    set_should_fail(true);

    // Perform the test that may cause a normal double fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }
}
