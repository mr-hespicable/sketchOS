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
    use x86_64::{structures::paging::Translate, VirtAddr};

    sketch_os::init(); //init idt

    let phys_mem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        bootinfo.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

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
