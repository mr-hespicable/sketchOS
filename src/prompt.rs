use alloc::{
    format,
    string::{String, ToString},
};

#[derive(Debug, Clone)]
pub struct Prompt {
    pub user: String,
    pub machine: String,
    pub prompt_row: usize,
    pub prompt_column: usize,
}

impl Prompt {
    pub fn new(user: String, machine: String) -> Prompt {
        let prompt_length = user.len() + machine.len() + 5;

        Prompt {
            user,
            machine,
            prompt_row: 0,
            prompt_column: prompt_length,
        }
    }

    pub fn len(&self) -> usize {
        self.user.len() + self.machine.len() + 5
    }

    pub fn is_empty(&self) -> bool {
        // WHY DO I EVEN NEED THIS. BY LOGIC len()
        // RETURNS A VALUE GREATER THAN 0
        false
    }

    pub fn prompt(&self) -> String {
        format!("{}@{} ~> ", self.user, self.machine).to_string() // `user@machine ~> `
    }
}
