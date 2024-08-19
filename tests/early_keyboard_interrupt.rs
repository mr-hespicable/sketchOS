#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm_const)]

use core::arch::asm;
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use sketch_os::{
    exit_qemu, gdt, hlt_loop,
    interrupts::{InterruptIndex, PICS},
    serial_print, serial_println, QemuExitCode,
};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("early_keyboard_interrupt::early_keyboard_interrupt...\t");

    early_keyboard_interrupt(); // send interrupt
    gdt::init();
    init_test_idt(); // init idt

    // if the test does not panic
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    hlt_loop();
}

fn early_keyboard_interrupt() {
    unsafe { x86_64::software_interrupt!(33) };
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[33].set_handler_fn(test_handler_interrupt_keyboard);
        idt
    };
}

extern "x86-interrupt" fn test_handler_interrupt_keyboard(_stack_frame: InterruptStackFrame) {
    serial_print!("\n{:#?}\n", _stack_frame); // print stack frame

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{:#?}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}
