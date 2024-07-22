#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use sketch_os::{allocator, memory::BootInfoFrameAllocator, print, println};
use spin::Mutex;

lazy_static! {
    pub static ref USER: Mutex<&'static str> = Mutex::new("user");
}

lazy_static! {
    pub static ref MACHINE: Mutex<&'static str> = Mutex::new("machine");
}

entry_point!(kernal_main);
#[no_mangle]
fn kernal_main(bootinfo: &'static BootInfo) -> ! {
    use sketch_os::memory;
    use x86_64::VirtAddr;

    sketch_os::init(); // init idt

    // start up memory
    let phys_mem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&bootinfo.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed"); // init heap

    /* MAIN CODE GOES HERE */
    print!("WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW");

    #[cfg(test)]
    test_main();

    sketch_os::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sketch_os::test_panic_handler(info)
}

//call this on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!();
    sketch_os::hlt_loop();
}
