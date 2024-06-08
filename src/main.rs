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
use spin::Mutex;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

use sketch_os::{allocator, memory::BootInfoFrameAllocator, println};

lazy_static! {
    pub static ref USER: Mutex<&'static str> = Mutex::new("user");
}

lazy_static! {
    pub static ref MACHINE: Mutex<&'static str> = Mutex::new("machine");
}

entry_point!(kernal_main);
fn kernal_main(bootinfo: &'static BootInfo) -> ! {
    use sketch_os::{draw_prompt, memory};
    use x86_64::VirtAddr;

    sketch_os::init(); // init idt

    let phys_mem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&bootinfo.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed"); // init heap

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    #[cfg(test)]
    test_main();

    println!("it did not crash?!");
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
