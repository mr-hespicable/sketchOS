#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;

use sketch_os::println;

lazy_static! {
    pub static ref USER: Mutex<&'static str> = Mutex::new("user");
}

lazy_static! {
    pub static ref MACHINE: Mutex<&'static str> = Mutex::new("machine");
}

entry_point!(kernal_main);
fn kernal_main(bootinfo: &'static BootInfo) -> ! {
    use sketch_os::{draw_prompt, memory};
    use x86_64::{
        structures::paging::{Page, Size4KiB},
        VirtAddr,
    };

    sketch_os::init(); //init idt

    let phys_mem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) }

    #[cfg(test)]
    test_main();

    println!("it did not crash!");
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
