use alloc::string::ToString;
use core::fmt::{Arguments, Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts;

use crate::prompt::Prompt;

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

// buffer type
struct Buffer {
    //screenchar repeated across BUFFER_WIDTH times, repeated down BUFFER_HEIGHT times.
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// writer data type
pub struct Writer {
    cursor_column: usize,  //the position of the cursor (column-wise).
    cursor_row: usize,     //the position of the row (row-wise).
    text_column: usize,    //the position of the text on the screen (column-wise).
    text_row: usize,       //the position of the text on the screen (row-wise).
    color_fg: Color,       //foreground color
    color_bg: Color,       //background color
    color_code: ColorCode, //the colorcode
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = {
        let color_fg = Color::Yellow; //set default value
        let color_bg = Color::Black;  //set default value

        Mutex::new(Writer {
            cursor_column: 0,
            cursor_row: 0,
            text_column: 0,
            text_row: 0,
            color_fg,
            color_bg,
            color_code: ColorCode::new(color_fg, color_bg),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        })
    };
}

impl Writer {
    /* WRiTING THINGS */
    pub fn write_byte(&mut self, byte: u8, row: usize, col: usize) {
        match byte {
            b'\n' => {
                self.new_line();
                self.move_cursor(Direction::Right, 1);
                self.move_text(Direction::Right, true);
            }
            byte => {
                if self.cursor_column != self.text_column {
                    // TODO: figure out how to move chars
                }

                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code,
                });

                self.move_cursor(Direction::Right, 1);
                self.move_text(Direction::Right, false);
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for i in 0..s.len() {
            let byte = s.as_bytes()[i];
            match byte {
                // printable ascii byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte, self.cursor_row, self.cursor_column),
                _ => self.write_byte(0xfe, self.cursor_row, self.cursor_column),
            }
        }
    }

    /* NEW LINE */
    fn new_line(&mut self) {
        let bottom_screen_index = BUFFER_HEIGHT - 1;
        if self.cursor_row == bottom_screen_index {
            self.shift_screen(Direction::Down);
            self.clear_line(bottom_screen_index);
            self.cursor_column = 0;
        } else {
            self.cursor_row += 1;
            self.cursor_column = 0;
        }
    }

    /* SCREEN FUNCTIONS */
    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_line(row);
        }
        self.draw_prompt("clear -> user", "machine");
    }

    fn clear_line(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_char: b' ',
                color_code: self.color_code,
            });
        }
    }

    fn shift_screen(&mut self, direction: Direction) {
        match direction {
            Direction::Down => {
                for row in 1..BUFFER_HEIGHT {
                    for col in 0..BUFFER_WIDTH {
                        let char: ScreenChar = self.buffer.chars[row][col].read();
                        self.buffer.chars[row - 1][col].write(char);
                    }
                }
                self.text_row -= 1;
            }
            Direction::Up => {
                for row in (0..BUFFER_HEIGHT - 1).rev() {
                    for col in 0..BUFFER_WIDTH {
                        let char: ScreenChar = self.buffer.chars[row][col].read();
                        self.buffer.chars[row + 1][col].write(char);
                    }
                }
                self.text_row += 1;
            }
            _ => panic!("can't put left or right here m8"),
        }
    }

    fn move_chars(&mut self, direction: Direction) {
        let cursor_row = self.cursor_row;
        let text_row = self.text_row;

        match direction {
            Direction::Left => {
                for row in self.cursor_row..self.text_row {
                    if cursor_row == text_row {
                        self.move_line(Direction::Left, self.cursor_column, self.text_column)
                    }
                    // moving text spans 1 row
                    else if row == cursor_row {
                        self.move_line(Direction::Left, self.cursor_column, BUFFER_WIDTH - 1)
                    }
                    // beginning of block of text
                    else if row == text_row {
                        self.move_line(Direction::Left, 0, self.text_column)
                    }
                    // end of block of text
                    else {
                        self.move_line(Direction::Left, 0, BUFFER_WIDTH - 1)
                    } // middle of block of text
                }
            }
            Direction::Right => {
                for row in (self.cursor_row..self.text_row).rev() {
                    if cursor_row == text_row {
                        self.move_line(Direction::Right, self.cursor_column, self.text_column)
                    }
                    // moving text spans 1 row
                    else if row == cursor_row {
                        self.move_line(Direction::Right, self.cursor_column, BUFFER_WIDTH - 1)
                    }
                    // beginning of block of text
                    else if row == text_row {
                        self.move_line(Direction::Right, 0, self.text_column)
                    }
                    // end of block of text
                    else {
                        self.move_line(Direction::Right, 0, BUFFER_WIDTH - 1)
                    } // middle of block of text
                }
            }
            _ => panic!("can't put up or down here m8"),
        }
    }

    fn move_line(&mut self, direction: Direction, left_col_index: usize, right_col_index: usize) {
        let row = self.cursor_row;

        match direction {
            Direction::Left => {
                for col in left_col_index..right_col_index {
                    let char = self.buffer.chars[row][col].read();

                    match col {
                        0 => self.buffer.chars[row - 1][BUFFER_WIDTH - 1].write(char),
                        _ => self.buffer.chars[row][col - 1].write(char),
                    }
                }
            }
            Direction::Right => {
                for col in (left_col_index..right_col_index).rev() {
                    let char = self.buffer.chars[row][col].read();

                    match col {
                        79 => self.buffer.chars[row + 1][0].write(char), // TODO: make this flexible
                        _ => self.buffer.chars[row][col + 1].write(char),
                    }
                }
            }
            _ => panic!("can't put up or down here m8"),
        }
    }

    fn delete_byte(&mut self) {
        if self.safe_to_delete() {
            match self.cursor_column {
                0 => {
                    self.buffer.chars[self.cursor_row - 1][BUFFER_WIDTH - 1].write(ScreenChar {
                        ascii_char: b' ',
                        color_code: self.color_code,
                    });
                }
                _ => {
                    self.buffer.chars[self.cursor_row][self.cursor_column - 1].write(ScreenChar {
                        ascii_char: b' ',
                        color_code: self.color_code,
                    });
                }
            }
            self.move_cursor(Direction::Left, 1);
            self.move_text(Direction::Left, false);
        }
    }

    fn safe_to_delete(&mut self) -> bool {
        //TODO: make this all work

        /*
        let prompt_length = *crate::PROMPT_LENGTH.lock();
        let prompt_row = *crate::PROMPT_ROW.lock();
        if self.cursor_column <= prompt_length && self.cursor_row == prompt_row {
            false
        } else {
            true
        }
        */
        true
    }

    fn move_text(&mut self, direction: Direction, newline_check: bool) {
        /*
         *   For the movement of self.text_[column, row].
         */
        match direction {
            Direction::Left => match self.text_column {
                0 => {
                    self.text_row -= 1;
                    self.text_column = BUFFER_WIDTH - 1;
                }
                _ => self.text_column -= 1,
            },
            Direction::Right => {
                if self.text_column == BUFFER_WIDTH - 1 {
                    self.text_row += 1;
                    self.text_column = 0;
                } else {
                    self.text_column += 1;
                }
            }
            _ => panic!(
                "can't put that as a direction... you put {:?} which doesn't make sense (i hope)",
                direction
            ),
        }

        if newline_check {
            self.text_row += 1
        }
    }

    /* END SCREEN FUNCTIONS */

    /* CURSOR FUNCTIONS*/
    fn move_cursor(&mut self, direction: Direction, iterations: usize) {
        for _ in 0..iterations {
            match direction {
                Direction::Up => {
                    if self.cursor_row == 0 {
                        self.shift_screen(Direction::Up)
                    }
                    self.cursor_row -= 1;
                }
                Direction::Down => {
                    if self.cursor_row == BUFFER_HEIGHT - 1 {
                        self.shift_screen(Direction::Down)
                    }
                    self.cursor_row += 1;
                }

                Direction::Left => match self.cursor_column {
                    0 => {
                        self.move_cursor(Direction::Up, 1);
                        self.cursor_column = BUFFER_WIDTH - 1;
                    }
                    _ => {
                        self.cursor_column -= 1;
                    }
                },
                Direction::Right => {
                    if self.cursor_column == BUFFER_WIDTH - 1 {
                        self.move_cursor(Direction::Down, 1);
                        self.cursor_column = 0;
                    } else {
                        self.cursor_column += 1;
                    }
                }
            }
        }
        self.draw_cursor();
    }

    fn draw_cursor(&mut self) {
        // make all chars not highlighted
        for row in 0..BUFFER_HEIGHT - 1 {
            for col in 0..BUFFER_WIDTH - 1 {
                let ascii_char = self.buffer.chars[row][col].read().ascii_char;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char,
                    color_code: ColorCode::new(self.color_fg, self.color_bg),
                });
            }
        }

        // highlight current cursor position
        let row = self.cursor_row;
        let col = self.cursor_column;

        let ascii_char = self.buffer.chars[row][col].read().ascii_char;

        self.buffer.chars[row][col].write(ScreenChar {
            ascii_char,
            color_code: ColorCode::new(self.color_bg, self.color_fg),
        });
    }
    /* END CURSOR FUNCTIONS */

    /* OTHERS */
    pub fn draw_prompt(&mut self, user: &str, machine: &str) {
        let prompt: Prompt = Prompt::new(user.to_string(), machine.to_string());

        self.write_string(&prompt.user as &str);
    }

    /* END OTHERS */
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

/***** MACROS *****/

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
macro_rules! print_byte {
    ($($arg:tt)*) => ($crate::vga_buffer::_print_byte(format_args!($($arg)*)));
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

#[macro_export]
macro_rules! draw_prompt {
    ($user:expr, $machine:expr) => {
        $crate::vga_buffer::_draw_prompt($user, $machine);
    };
}

/*
* --------------------------
* HIDDEN FNS: DO NOT TOUCH
* --------------------------
*/

#[doc(hidden)]
pub fn _print(args: Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_byte(args: Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })
}

#[doc(hidden)]
pub fn _clear() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.clear_screen();
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
        writer.move_cursor(Direction::Left, 1);
    });
}

#[doc(hidden)]
pub fn _move_cursor_right() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.move_cursor(Direction::Right, 1);
    });
}

#[doc(hidden)]
pub fn _move_chars_left() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.move_chars(Direction::Left);
    });
}

#[doc(hidden)]
pub fn _move_chars_right() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.move_chars(Direction::Right);
    });
}

#[doc(hidden)]
pub fn _draw_prompt(user: &str, machine: &str) {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.draw_prompt(user, machine);
    })
}
