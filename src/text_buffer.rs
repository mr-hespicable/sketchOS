use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt, str};

use crate::clear;

#[derive(Default)]
pub struct TextBuffer {
    information: Vec<u8>,
}

impl TextBuffer {
    pub fn new() -> Self {
        TextBuffer {
            information: Default::default(),
        }
    }

    /// Converts a `&str` into an iterator over the bytes of the `&str`, then pushes each byte into the buffer.
    pub fn append_str(&mut self, string: &str) {
        for byte in string.bytes() {
            self.append_byte(byte);
        }
    }

    /// Pushes a byte into the buffer
    pub fn append_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.process_command();
        }
        self.push(byte);
    }

    fn push(&mut self, byte: u8) {
        self.information.push(byte);
    }

    /// Pops & returns the last index of the buffer.
    pub fn pop(&mut self) -> Option<u8> {
        self.information.pop()
    }

    /// Returns the number of elements in the text buffer, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.information.len()
    }

    /// Says whether the text buffer is empty or not
    pub fn is_empty(&self) -> bool {
        self.information.is_empty()
    }

    pub fn as_string(&self) -> String {
        if !&self.information.is_ascii() {
            panic!("command contains non-ascii bytes")
        }
        String::from_utf8(self.information.clone()).unwrap()
    }

    pub fn process_command(&self) -> Option<CommandResult> {
        let command = self.as_string();

        if self.is_empty() {
            return None;
        }

        if command.starts_with("echo ") {
            let body: &str = command.splitn(2, " ").collect::<Vec<_>>()[1];
            let mut data_bytes = body.as_bytes().to_vec();
            data_bytes.push(b'\n');

            return Some(CommandResult { data_bytes });
        }

        if command.starts_with("clear") {
            clear!();
        }

        None
    }
}

impl fmt::Display for TextBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.information)
    }
}

#[derive(Default)]
pub struct CommandResult {
    data_bytes: Vec<u8>,
}

impl CommandResult {
    pub fn as_string(&self) -> String {
        if !&self.data_bytes.is_ascii() {
            panic!("command contains non-ascii bytes")
        }
        String::from_utf8(self.data_bytes.clone()).unwrap()
    }
}

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.data_bytes)
    }
}
