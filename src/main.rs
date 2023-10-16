#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sketch_os::{clear, print, println};

//don't mangle this function's name (basically, don' fuck it up)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n10\n11\n12\n12\n12\n12n12\n12\n12");

    sketch_os::init(); //init idt

    #[cfg(test)]
    test_main();
    sketch_os::hlt_loop();
}

//call this on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    sketch_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sketch_os::test_panic_handler(info)
}
