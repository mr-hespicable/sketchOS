#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(const_mut_refs)]

use crate::prompt::Prompt;
use alloc::string::ToString;
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;

extern crate alloc;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod process_command;
pub mod prompt;
pub mod serial;
pub mod text_buffer;
pub mod vga_buffer;

lazy_static! {
    pub static ref PROMPT: Mutex<Prompt> =
        Mutex::new(Prompt::new("user".to_string(), "machine".to_string()));
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("\nRunning {} tests", tests.len());
    for test in tests {
        test.run();
    }
    serial_println!();

    exit_qemu(QemuExitCode::Success);
}

//call this on panic
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[allow(unused_imports)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernal_main);

#[cfg(test)]
/// entry point for `cargo test`
fn test_kernal_main(_boot_info: &'static BootInfo) -> ! {
    init(); //init idt
    test_main(); //call tests
    hlt_loop(); //hang
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10, //2 is success
    Failed = 0x11,  //3 is failure
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Intializes the GDT, IDT, and enables interrupts.
pub fn init() {
    //initialization of the global descriptor table
    gdt::init();
    //initialization of the 8259 PIC
    unsafe { interrupts::PICS.lock().initialize() };
    //initialization of the interrupt descriptor table
    interrupts::init_idt();
    //enable interrupts
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
