#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"hello!";
//don't mangle this function's name (basically, don't fuck it up)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //_start() signifies that this is the entry point
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

//call this on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
