use alloc::{string::String, vec};

use crate::{clear, process_command::CommandResult};

pub fn clear() -> CommandResult {
    let empty_result = CommandResult { data_bytes: vec![] };
    let clear_screen = true;

    clear!();
    empty_result
}
