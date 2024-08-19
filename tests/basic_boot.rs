#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sketch_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sketch_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
