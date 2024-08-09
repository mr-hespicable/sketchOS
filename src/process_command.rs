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
    pub flags: ResultFlags,
}

#[derive(Default, Clone, Debug)]
pub struct ResultFlags {
    pub contains_result: bool,
    pub clear_screen: bool,
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

impl ResultFlags {
    pub fn new() -> Self {
        Self {
            contains_result: true,
            clear_screen: false,
        }
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
        _ => CommandResult {
            data_bytes: vec![],
            flags: ResultFlags {
                contains_result: false,
                clear_screen: false,
            },
        },
    }
}
