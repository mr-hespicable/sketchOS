#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::{structures::paging::PageTable, VirtAddr};

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
    use sketch_os::{draw_prompt, memory::active_level_4_table};
    sketch_os::init(); //init idt

    let phys_mem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + bootinfo.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 Entry {}: {:?}", i, entry);
                }
            }
        }
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
