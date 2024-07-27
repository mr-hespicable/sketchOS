use alloc::{string::String, vec::Vec};

use super::CommandResult;

pub fn echo(command: &String) -> CommandResult {
    let body: &str = command.splitn(2, " ").collect::<Vec<_>>()[1];
    let response = body.as_bytes().to_vec();

    CommandResult {
        data_bytes: response,
    }
}
