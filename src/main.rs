#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

//don't mangle this function's name (basically, don' fuck it up)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //_start() signifies that this is the entry point
    vga_buffer::test();

    loop {}
}

//call this on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
