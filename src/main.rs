#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::{mem::transmute, panic::PanicInfo};
use sketch_os::{
    draw_prompt,
    memory::{self},
    println, MACHINE, USER,
};
use x86_64::{
    registers::control::Cr3, 
    structures::paging::{
        PageTable, 
        Translate,
    }, 
    VirtAddr};

entry_point!(kernal_main);
fn kernal_main(boot_info: &'static BootInfo) -> ! {
    sketch_os::init(); //init idt

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!(" {:?} -> {:?} ", virt, phys);
    }
    //draw_prompt!(&*USER.lock(), &*MACHINE.lock());

    println!();

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
