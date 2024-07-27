use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::PROMPT;

use super::CommandResult;

pub fn iam(command: &String) -> CommandResult {
    let body: &str = command.splitn(2, " ").collect::<Vec<_>>()[1];
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
    }
}
