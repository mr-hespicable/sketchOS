#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use sketch_os::{exit_qemu, serial_print, serial_println, QemuExitCode};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static mut SHOULD_FAIL: bool = false;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    sketch_os::gdt::init();

    test_main();

    panic!("Execution continued after stack overflow");
}

//tests
#[test_case]
fn test_stack_overflow() {
    serial_println!("testing stack");
    stack_overflow();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

#[test_case]
fn test_double_fault() {
    set_should_fail(true);
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }
}
//end of tests

//custom panic stuff
fn should_fail() -> bool {
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
