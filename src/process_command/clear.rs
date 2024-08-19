use alloc::vec;

use crate::{clear, process_command::CommandResult};

pub fn clear() -> CommandResult {
    clear!();
    CommandResult {
        data_bytes: vec![],
        flags: super::ResultFlags {
            contains_result: false,
            clear_screen: true,
        },
    }
}
