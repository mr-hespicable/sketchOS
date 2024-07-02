use alloc::{
    format,
    string::{String, ToString},
};

#[derive(Debug)]
pub struct Prompt {
    pub user: String,
    pub machine: String,
    pub prompt_text: String,
    pub prompt_location: (usize, usize),
}

impl Prompt {
    pub fn new(user: String, machine: String) -> Prompt {
        let prompt = format!("{}@{} ~>", user, machine).to_string(); // user@machine>>

        Prompt {
            user,
            machine,
            prompt_text: prompt,
            prompt_location: (0, 0),
        }
    }
}
