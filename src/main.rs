#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(asm_const)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::arch::asm;
use core::panic::PanicInfo;
use sketch_os::{allocator, draw_prompt, memory::BootInfoFrameAllocator, println};
use x86_64::instructions::interrupts::without_interrupts;

entry_point!(kernal_main);
#[no_mangle]
fn kernal_main(bootinfo: &'static BootInfo) -> ! {
    // sketch_os::init();

    use sketch_os::gdt;
    use sketch_os::interrupts;
    // initialization of the interrupt descriptor table
    interrupts::init_idt();

    // initialization of the global descriptor table
    gdt::init();

    // initialization of the 8259 PIC programmable interrupt controller?
    unsafe { interrupts::PICS.lock().initialize() };

    x86_64::instructions::interrupts::enable();

    use sketch_os::memory;
    use x86_64::VirtAddr;

    // start up memory
    let phys_mem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&bootinfo.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed"); // init heap

    /* MAIN CODE GOES HERE */
    draw_prompt!();
    //let coords: (usize, usize) = (fake_prompt.prompt_row, fake_prompt.prompt_column);
    //print!("{:?}", coords);

    /* main code end */

    #[cfg(test)]
    test_main();

    sketch_os::hlt_loop()
}

//call this on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    sketch_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sketch_os::test_panic_handler(info)
}
