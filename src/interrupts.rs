use crate::process_command::CommandResult;
use crate::vga_buffer::TEXT_BUFFER;
use crate::{backspace, draw_prompt, gdt, hlt_loop, print, println};
use alloc::string::ToString;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = //set PIC offsets
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

//PIC things
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! { //set idt table
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();//new init of interrupt descriptor table
        idt.breakpoint.set_handler_fn(handler_breakpoint);
        unsafe {
            idt.double_fault.set_handler_fn(handler_double_fault)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.page_fault.set_handler_fn(handler_page_fault);
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(handler_interrupt_timer);

        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(handler_interrupt_keyboard);

        idt
    };
}

pub fn init_idt() {
    //init idt
    IDT.load();
}
extern "x86-interrupt" fn handler_breakpoint(stack_frame: InterruptStackFrame) {
    //breakpoint handle
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handler_page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

extern "x86-interrupt" fn handler_double_fault(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handler_interrupt_timer(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn handler_interrupt_keyboard(_stack_frame: InterruptStackFrame) {
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut port = Port::new(0x60);
    let mut keyboard = KEYBOARD.lock();

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        //write keyboard thing to screen
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    if scancode == 14 {
                        /*
                         *
                         * backspace
                         *
                         */
                        TEXT_BUFFER.lock().pop();
                        backspace!();
                    } else if scancode == 28 {
                        /*
                         *
                         * return
                         *
                         */
                        let result: CommandResult = TEXT_BUFFER.lock().process_command();
                        print!("{}", result.as_string());
                        TEXT_BUFFER.lock().clear_buf(); // clear the buffer
                        draw_prompt!();
                    } else {
                        // all else
                        TEXT_BUFFER
                            .lock()
                            .append_str(character.to_string().as_str());
                        print!("{}", character);
                    }
                }
                DecodedKey::RawKey(_) => {
                    // do nothing
                }
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
