use alloc::{fmt, string::String, vec, vec::Vec};

use crate::text_buffer::TextBuffer;

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

        if command == "echo" {
            Some(CommandResult {
                data_bytes: vec![b'A'],
            })
        } else {
            None
        }
    }
}
