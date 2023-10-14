#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sketch_os::{print, println};

//don't mangle this function's name (basically, don' fuck it up)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world!");

    sketch_os::init(); //init idt

    #[allow(unconditional_recursion)]
    fn ovrflow() {
        ovrflow();
    }
    ovrflow();

    #[cfg(test)]
    test_main();

    print!("it did not crash!");

    #[allow(clippy::empty_loop)]
    loop {}
}

//call this on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sketch_os::test_panic_handler(info)
}
