#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sketch_os::{print, println};
use sketch_os::vga_buffer::_flip_current;

mod prompt;
mod vga_buffer;

//don't mangle this function's name (basically, don' mess it up)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    sketch_os::init(); //init idt
    
    //print!("1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDE\n\n");
    
    let user = "user";
    let machine = "workspace";
    
    prompt::draw_prompt(user, machine);

    // println!("{:?}", proper_prompt);
    // print!("{}", proper_prompt);
    _flip_current(1000, 1000); //draw cursor

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
