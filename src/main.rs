#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use prompt::draw_prompt;
use sketch_os::{draw_prompt, print, println, vga_buffer};

mod prompt;

//don't mangle this function's name (basically, don' mess it up)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    sketch_os::init(); //init idt
    
    //print!("1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDE\n\n");
    
    let user = "user";
    let machine = "workspace";
    
    draw_prompt!();

    // println!("\n1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n0");

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
