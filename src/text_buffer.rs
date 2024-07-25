use alloc::vec::Vec;
use core::{fmt, str};

use crate::process_commmand;

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
            self.push(byte);
        }
    }

    /// Pushes a byte into the buffer
    pub fn append_byte(&mut self, byte: u8) {
        if byte == b'\n' {}
        self.push(byte);
    }

    fn push(&mut self, byte: u8) {
        self.information.push(byte);
    }

    /// Returns the number of elements in the text buffer, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.information.len()
    }

    /// Says whether the text buffer is empty or not
    pub fn is_empty(&self) -> bool {
        self.information.is_empty()
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

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.data_bytes)
    }
}

trait ProcessCommand {
    fn process_command(&self, command: String) -> Option<CommandResult>;
}

impl ProcessCommand for TextBuffer {
    fn process_command(&self, command: String) -> Option<CommandResult> {
        if self.is_empty() {
            return None;
        }

        if command.starts_with("echo ") {
            let body: &str = command.splitn(2, " ").collect::<Vec<_>>()[1];

            Some(CommandResult {
                data_bytes: body.as_bytes().to_vec(),
            })
        } else {
            None
        }
    }
}
