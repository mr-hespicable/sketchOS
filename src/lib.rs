#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

lazy_static! {
    pub static ref USER: Mutex<&'static str> = Mutex::new("user");
    pub static ref MACHINE: Mutex<&'static str> = Mutex::new("machine");
    pub static ref PROMPT_LENGTH: Mutex<usize> = Mutex::new(0);
    pub static ref PROMPT_ROW: Mutex<usize> = Mutex::new(0);
}

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod prompt;
pub mod serial;
pub mod vga_buffer;

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
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

//call this on panic
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
entry_point!(test_kernal_main);

#[cfg(test)]
fn test_kernal_main(_boot_info: &'static BootInfo) -> ! {
    //entry point for 'cargo test'
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

pub fn init() {
    //initialization of the global descriptor table
    gdt::init();
    //initialization of the interrupt descriptor table
    interrupts::init_idt();
    //initialization of the 8259 PIC
    unsafe { interrupts::PICS.lock().initialize() };
    //enable interrupts
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
