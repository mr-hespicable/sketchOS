use core::str::FromStr;

use alloc::{
    format,
    string::{String, ToString},
};

use crate::println;

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

fn _main() {
    let prompt = Prompt::new(
        String::from_str("user").unwrap(),
        String::from_str("machine").unwrap(),
    );

    println!("{:?}", prompt);
}
