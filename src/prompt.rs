use core::str::{from_utf8, from_utf8_unchecked};

use sketch_os::{print, println};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Prompt<'a>(&'a str);

struct Buffer {
    chars: [0u8, 256],
}

impl Prompt<'_> {
    fn new<'a>(user: &'a str, machine: &'a str) -> Prompt<'a> {
    let lit = user, machine;
    
        Prompt(final_prompt)
    }
}

pub fn make_prompt(user: &str, machine: &str) {
    let prompt = Prompt::new(user, machine);
    println!("{:?}", prompt);
}

