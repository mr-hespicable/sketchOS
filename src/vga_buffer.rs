use core::fmt::{Arguments, Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts::{self, without_interrupts}; #[allow(dead_code)]
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

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Cyan,
            4 => Color::Red,
            5 => Color::Magenta,
            6 => Color::Brown,
            7 => Color::LightGray,
            8 => Color::DarkGray,
            9 => Color::LightBlue,
            10 => Color::LightGreen,
            11 => Color::LightCyan,
            12 => Color::LightRed,
            13 => Color::Pink,
            14 => Color::Yellow,
            15 => Color::White,
            _ => Color::Black, // Default to Black if the value is out of range
        }
    }
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
    cursor_column: usize,    //the position of the cursor (column-wise).
    cursor_row: usize,       //the position of the row (row-wise).
    write_column: usize,     //the position of the text on the screen (column-wise).
    write_row: usize,        //the position of the text on the screen (row-wise).
    color_fg: Color,         //foreground color
    color_bg: Color,         //background color
    color_code: ColorCode,   //the colorcode
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = {
        let color_fg = Color::Yellow; //set default value
        let color_bg = Color::Black; //set default value

        Mutex::new(Writer {
            cursor_column: 0,
            cursor_row: 0,
            write_column: 0,
            write_row: 0,
            color_fg,
            color_bg,
            color_code: ColorCode::new(color_fg, color_bg),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        })
    };
}
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        self.flip_char(self.cursor_row, self.cursor_column, 1);
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.cursor_column >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.cursor_row;
                let col = self.cursor_column;

                if self.cursor_column != self.write_column {
                    self.move_chars(1);
                }

                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code,
                });
                self.cursor_column += 1;
                self.write_column += 1;
                self.flip_char(self.cursor_row, self.cursor_column, 0);
            }
        }
    }

    pub fn move_chars(&mut self, direction: i32) { //called from another cursor move func, never
                                                   //called directly
        //0 means move left, 1 means move right
        if direction == 0 {
            if self.cursor_row == self.write_row { //if moving text is a one liner
                let row = self.cursor_row;
                for col in self.cursor_column..self.write_column + 1 { // +1 is so that the last
                    let character = self.buffer.chars[row][col].read();// char isn't left behind
                    if col == 0 {
                        self.buffer.chars[row-1][BUFFER_WIDTH-1].write(character);
                    } else {
                        self.buffer.chars[row][col-1].write(character);
                    }
                }
            } else if self.write_column == 0 { //if we've just moved back to an old line
                let row = self.cursor_row;
                for col in self.cursor_column..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row][col-1].write(character);
                }
                self.buffer.chars[row][79].write( ScreenChar { //delete that last char on the row
                    ascii_char: b' ', 
                    color_code: self.color_code,
                });
                self.write_column = 80
            } else { //if the moving text spans across multiple rows
                for row in self.cursor_row..self.write_row + 1 {
                    if row == self.write_row { // end of text block
                        for col in 0..self.write_column + 1 { // +1: see line 107
                            let character = self.buffer.chars[row][col].read();
                            if col == 0 {
                                self.buffer.chars[row-1][BUFFER_WIDTH-1].write(character);
                            } else {
                                self.buffer.chars[row][col-1].write(character);
                            }
                        }
                    } else if row == self.cursor_row { //beginning of text block
                        for col in self.cursor_column..BUFFER_WIDTH {
                            let character = self.buffer.chars[row][col].read();
                            if col == 0 {
                                self.buffer.chars[row-1][BUFFER_WIDTH-1].write(character);
                            } else {
                                self.buffer.chars[row][col-1].write(character);
                            }
                        }
                    } else { //middle of text block
                        for col in 0..BUFFER_WIDTH {
                            let character = self.buffer.chars[row][col].read();
                            if col == 0 {
                                self.buffer.chars[row-1][BUFFER_WIDTH-1].write(character);
                            } else {
                                self.buffer.chars[row][col-1].write(character);
                            }
                        } 
                    }
                }
            }
        } else if direction == 1 {
            if self.cursor_row == self.write_row {
                let row = self.cursor_row;
                for col in (self.cursor_column..self.write_column).rev() {
                    let character = self.buffer.chars[row][col].read();
                    if col == BUFFER_WIDTH-1 {
                        self.buffer.chars[row+1][0].write(character);
                    } else {
                        self.buffer.chars[row][col+1].write(character);
                    }
                }
            }
        } else { //because i will 100% make this mistake
            panic!("non binary value inputted into move_chars")
        }
    }

    pub fn delete_byte(&mut self) {
        let row = self.cursor_row;
        let col = self.cursor_column;
        let color_code = self.color_code;

        self.flip_char(self.cursor_row, self.cursor_column, 1); //unflip cursor
        
        if col == 0 {
            self.buffer.chars[row - 1][BUFFER_WIDTH - 1].write(ScreenChar {
                ascii_char: b' ',
                color_code,
            });

            if self.cursor_column != self.write_column {
                self.move_chars(0);
            }

            self.cursor_row -= 1;
            self.cursor_column = BUFFER_WIDTH - 1;

            if self.write_column == 0 {
                self.write_row -= 1;
                self.write_column = BUFFER_WIDTH - 1;
            } else {
                self.write_column -= 1;
            }

            self.flip_char(self.cursor_row, self.cursor_column, 0); //flip cursor
        } else {
            self.buffer.chars[row][col - 1].write(ScreenChar {
                ascii_char: b' ',
                color_code,
            });

            if self.cursor_column != self.write_column {
                self.move_chars(0);
            }

            self.cursor_column -= 1;
            self.write_column -= 1;

            self.flip_char(self.cursor_row, self.cursor_column, 0); //flip cursor
        }
    }

    fn new_line(&mut self) {
        if self.cursor_row + 2 > BUFFER_HEIGHT {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
        } else {
            self.cursor_row += 1;
            self.write_row += 1;
        }
        self.clear_row(BUFFER_HEIGHT - 1);

        self.cursor_column = 0;
        self.write_column = 0;
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
        let row = self.cursor_row;
        let col = self.cursor_column;
        //0 is left, 1 is right
        if direction == 0 {
            self.flip_char(self.cursor_row, self.cursor_column, 1); //unflip cursor
            if col == 0 {
                self.cursor_row -= 1;
                self.cursor_column = BUFFER_WIDTH - 1;
            } else {
                self.cursor_column -= 1;
            }

            self.flip_char(self.cursor_row, self.cursor_column, 0); //flip cursor
        } else if direction == 1 {
            self.flip_char(self.cursor_row, self.cursor_column, 1); //flip cursor
            if col == BUFFER_WIDTH - 1 {
                self.cursor_row += 1;
                self.cursor_column = 0;
            } else {
                self.cursor_column += 1;
            }
            self.flip_char(self.cursor_row, self.cursor_column, 0); //flip cursor
        }
    }

    pub fn flip_char(&mut self, raw_row: usize, raw_col: usize, direction: i32) {
        //0 is inverted, 1 is normal
        let mut row;
        let mut  col;

        if raw_row == 1000 && raw_col == 1000 {
            row = self.cursor_row;
            col = self.cursor_column;
        } else {
            row = raw_row;
            col = raw_col;
        }

        if col >= BUFFER_WIDTH {
            col -= BUFFER_WIDTH
        }

        let character = self.buffer.chars[row][col].read();

        let fg = self.color_fg;
        let bg = self.color_bg;

        if direction == 1 {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_char: character.ascii_char,
                color_code: ColorCode::new(fg, bg),
            });
        } else {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_char: character.ascii_char,
                color_code: ColorCode::new(bg, fg),
            });
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
        writer.cursor_column = 0;
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

#[doc(hidden)]
pub fn _flip_current(row: usize, col: usize) {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.flip_char(row, col, 0);
    });
}
