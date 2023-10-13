#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    sketch_os::init(); //init idt
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sketch_os::test_panic_handler(info)
}

#[test_case]
fn test_breakpoint() {
    x86_64::instructions::interrupts::int3();
}

//double fault will go here
#[test_case]
fn test_double_fault() {
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }
}
