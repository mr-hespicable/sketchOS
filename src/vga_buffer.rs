use crate::print;
use core::fmt::{Arguments, Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

//screen character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//writer type
pub struct Writer {
    column_position: usize, //the position of the cursor (column-wise).
    row_position: usize,    //the position of the row (row-wise).
    text_column: usize,     //the position of the text on the screen (column-wise).
    text_row: usize,        //the position of the text on the screen (row-wise).
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        text_column: 0,
        text_row: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                if self.column_position != self.text_column {
                    self.move_chars(1);
                }

                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code,
                });
                self.column_position += 1;
                self.text_column += 1;
            }
        }
    }

    pub fn move_chars(&mut self, direction: i32) {
        //0 means move left, 1 means move right
        if direction == 0 {
            for row in (self.row_position..self.text_row + 1) {
                for col in (self.column_position..self.text_column) {
                    let character = self.buffer.chars[row][col].read();
                    if col - 1 >= 0 {
                        self.buffer.chars[row][col - 1].write(character);
                    } else {
                        self.buffer.chars[row - 1][0].write(character);
                        self.text_row -= 1;
                    }
                }
            }
        } else if direction == 1 {
            for row in (self.row_position..self.text_row + 1).rev() {
                for col in (self.column_position..self.text_column).rev() {
                    let character = self.buffer.chars[row][col].read();
                    if col + 1 < BUFFER_WIDTH {
                        self.buffer.chars[row][col + 1].write(character);
                    } else {
                        self.buffer.chars[row + 1][0].write(character);
                    }
                }
            }
            self.buffer.chars[self.text_row][self.text_column].write(ScreenChar {
                ascii_char: b' ',
                color_code: self.color_code,
            });
        }
    }

    pub fn delete_byte(&mut self) {
        let row = self.row_position;
        let col = self.column_position;
        let color_code = self.color_code;

        if col == 0 {
            self.row_position -= 1;
            self.text_row -= 1;
            self.column_position = BUFFER_WIDTH - 1;
            self.text_column = BUFFER_WIDTH - 1;

            self.buffer.chars[self.row_position][self.column_position].write(ScreenChar {
                ascii_char: b' ',
                color_code,
            });
            self.move_chars(0);
        } else {
            self.buffer.chars[row][col - 1].write(ScreenChar {
                ascii_char: b' ',
                color_code,
            });

            self.column_position -= 1;
            self.text_column -= 1;
            self.move_chars(0);
        }
    }

    fn new_line(&mut self) {
        if self.row_position + 2 > BUFFER_HEIGHT {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
        } else {
            self.row_position += 1;
            self.text_row += 1;
        }
        self.clear_row(BUFFER_HEIGHT - 1);

        self.column_position = 0;
        self.text_column = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ascii byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //if not... this.
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn move_cursor(&mut self, direction: i32) {
        // DO NOT CHANGE TEXT POSITION HERE
        let col = self.column_position;

        if direction == 0 && col > 0 {
            self.column_position -= 1;
        } else if direction == 0 && col == 0 {
            self.row_position -= 1;
            self.column_position = BUFFER_WIDTH - 1;
        } else if direction == 1 && col < self.text_column {
            self.column_position += 1;
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! clear {
    () => {
        $crate::vga_buffer::_clear();
    };
}

#[macro_export]
macro_rules! backspace {
    () => {
        $crate::vga_buffer::_delete();
    };
}

#[macro_export]
macro_rules! move_cursor {
    (0) => {
        $crate::vga_buffer::_move_cursor_left();
    };
    (1) => {
        $crate::vga_buffer::_move_cursor_right();
    };
}

#[macro_export]
macro_rules! move_chars {
    (0) => {
        $crate::vga_buffer::_move_chars_left();
    };
    (1) => {
        $crate::vga_buffer::_move_chars_right();
    };
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    interrupts::without_interrupts(|| {
        //no interrupt can occur when writer is locked
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _clear() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        for row in 0..BUFFER_HEIGHT {
            writer.clear_row(row);
        }
        writer.column_position = 0;
    });
}

#[doc(hidden)]
pub fn _delete() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.delete_byte();
    });
}

#[doc(hidden)]
pub fn _move_cursor_left() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.move_cursor(0);
    });
}

#[doc(hidden)]
pub fn _move_cursor_right() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.move_cursor(1);
    });
}

#[doc(hidden)]
pub fn _move_chars_left() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.move_chars(0);
    });
}

#[doc(hidden)]
pub fn _move_chars_right() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.move_chars(1);
    });
}
