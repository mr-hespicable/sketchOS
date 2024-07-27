use clear::clear;
use echo::echo;
use iam::iam;
use math::math;
use whoami::whoami;

use crate::alloc::{fmt, string::String, vec, vec::Vec};

pub mod clear;
pub mod echo;
pub mod iam;
pub mod math;
pub mod whoami;

#[derive(Default, Clone, Debug)]
pub struct CommandResult {
    pub data_bytes: Vec<u8>,
}

impl CommandResult {
    pub fn as_string(&self) -> String {
        if !&self.data_bytes.is_ascii() {
            panic!("command contains non-ascii bytes")
        }
        String::from_utf8(self.data_bytes.clone()).unwrap()
    }
}

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.data_bytes)
    }
}

pub fn process_command(command: String) -> CommandResult {
    let prefix = command.splitn(2, " ").collect::<Vec<_>>()[0];

    match prefix {
        "echo" => echo(&command),
        "clear" => clear(),
        "whoami" => whoami(),
        "iam" => iam(&command),
        "math" => math(&command),
        // "exit" => todo!(), //SIGKILL,
        _ => {
            let continue_on = false;
            CommandResult { data_bytes: vec![] }
        }
    }
}
