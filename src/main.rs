use core::panic::PanicInfo;

//panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_std]

fn main() {}
