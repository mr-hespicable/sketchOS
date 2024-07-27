use super::CommandResult;
use crate::PROMPT;

pub fn whoami() -> CommandResult {
    CommandResult {
        data_bytes: PROMPT.lock().user.as_bytes().to_vec(),
    }
}
