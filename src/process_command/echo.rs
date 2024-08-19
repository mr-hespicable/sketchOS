use alloc::vec::Vec;

use super::{CommandResult, ResultFlags};

pub fn echo(command: &str) -> CommandResult {
    let command = command.splitn(2, " ").collect::<Vec<_>>();
    let body = command.get(1);
    let response = body.unwrap_or(&"").as_bytes().to_vec();

    CommandResult {
        data_bytes: response,
        flags: ResultFlags::new(),
    }
}
