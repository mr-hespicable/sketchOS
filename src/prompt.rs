use sketch_os::{print, println};

#[derive(Debug)]
struct Prompt<'a> {
    user: &'a str,
    machine: &'a str,
}

impl<'a> Prompt<'a> {
    fn new(user: &'a str, machine: &'a str) -> Prompt<'a> {

        Prompt {
            user, 
            machine
        }

    }
}

pub fn make_prompt(user: &str, machine: &str) {
    let prompt = Prompt::new(user, machine);
    println!("{:?}", prompt);
}

