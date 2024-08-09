use alloc::{format, string::ToString, vec::Vec};

use crate::PROMPT;

use super::CommandResult;

pub fn iam(command: &str) -> CommandResult {
    let command = command.splitn(2, " ").collect::<Vec<_>>();
    let body = command.get(1).unwrap_or(&"");
    let response;

    if body.len() > 16 {
        response = "username cannot be more than 16 chars".to_string();
    } else if body.len() < 3 {
        response = "username cannot be less than 3 chars".to_string();
    } else {
        PROMPT.lock().user = body.to_string();
        response = format!("you are now {}", PROMPT.lock().user);
    }

    CommandResult {
        data_bytes: response.as_bytes().to_vec(),
        flags: super::ResultFlags::new(),
    }
}
