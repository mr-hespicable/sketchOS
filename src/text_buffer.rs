use alloc::{string::String, vec::Vec};
use core::{fmt, str};

use crate::process_command::{process_command, CommandResult};

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

    pub fn process_command(&mut self) -> CommandResult {
        let command = self.as_string();

        let mut command_result = process_command(command);

        let flags = &command_result.flags;
        if flags.clear_screen {
            // clear screen
            command_result
        } else if flags.contains_result && !flags.clear_screen {
            // prints something to stdout (echo, whoami, math)
            command_result.data_bytes.push(b'\n');
            command_result.data_bytes.insert(0, b'\n');
            command_result
        } else {
            // normal `return` function
            command_result.data_bytes.push(b'\n');
            command_result
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
        String::from_utf8(self.information.clone()).unwrap()
    }

    /// Clears the buffer, returning the raw contents of the buffer before it was cleared in its place.
    pub fn clear_buf(&mut self) -> Vec<u8> {
        self.information = Default::default();
        self.information.clone()
    }
}

impl fmt::Display for TextBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.information)
    }
}
