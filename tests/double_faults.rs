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
    sketch_os::init();
    test_main();
    #[allow(clippy::empty_loop)]
    loop {}
}

fn should_fail() -> bool {
    //hi
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

//tests

#[test_case]
fn stack_overflow() {
    set_should_fail(true);
    cause_stack_overflow();
}

#[allow(unconditional_recursion)]
fn cause_stack_overflow() {
    cause_stack_overflow();
    volatile::Volatile::new(0).read();
}

#[test_case]
fn double_fault() {
    set_should_fail(true);
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }
}
//end of tests
