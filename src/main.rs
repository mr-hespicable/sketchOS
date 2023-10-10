#![no_main]
#![no_std]

use core::panic::PanicInfo;
mod vga_buffer;

//don't mangle this function's name (basically, don' fuck it up)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("some panic msg");
    loop {}
}

//call this on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{_info}");
    loop {}
}
