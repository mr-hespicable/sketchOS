use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); //variable idt is now a new initialization
                                                       //of the interrupt descriptor table
        idt.breakpoint.set_handler_fn(handler_breakpoint);
        idt.double_fault.set_handler_fn(handler_double_fault);
        idt
    };
}

pub fn init_idt() {
    IDT.load(); //loads lazy static
}

extern "x86-interrupt" fn handler_breakpoint(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handler_double_fault(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
