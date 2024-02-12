use crate::print;
use core::str::from_utf8;

#[derive(Debug)]
pub struct Prompt<'a>(&'a str);

impl<'a> Prompt<'a> {
    fn new(user: &'a str, machine: &'a str) -> [u8; 256] { let mut prompt_array: [u8; 256] = [0; 256];
        let user_bytes: &[u8] = user.as_bytes(); //&[u8]
        let machine_bytes: &[u8] = machine.as_bytes(); //&[u8]
        let separator_bytes: &[u8] = "@".as_bytes();
        let suffix_bytes: &[u8] = ">>".as_bytes();
        //@ = 64 as bytes
        //> = 62 as bytes
        //goes `user@machine>>`
        prompt_array[..user_bytes.len()].copy_from_slice(user_bytes); //add username
        prompt_array[
            user_bytes.len()..user_bytes.len()+1
        ].copy_from_slice(separator_bytes); //add separator
        prompt_array[
            user_bytes.len()+1..user_bytes.len()+machine_bytes.len()+1
        ].copy_from_slice(machine_bytes); //add machine
        prompt_array[
            user_bytes.len()+machine_bytes.len()+1..user_bytes.len()+machine_bytes.len()+suffix_bytes.len()+1
        ].copy_from_slice(suffix_bytes);//add suffix
                                        
        prompt_array //return prompt as [u8; 256]
    }
}

pub fn draw_prompt(user: &str, machine: &str) -> usize {
    let prompt_bytes = Prompt::new(user, machine);
    let mut prompt_length: usize = 0;

    for byte in prompt_bytes {
        match byte {
            0 => continue,
            _ => {
                let borrowed = &[byte];
                let test = from_utf8(borrowed).unwrap();
                print!("{}", from_utf8(borrowed).unwrap());
                prompt_length += 1;
            },
        }
    }
    print!(" ");

    prompt_length

}

pub fn safe_to_delete(start_row: usize, current_row: usize, col: usize, prompt_length: usize) -> bool {
    unsafe {
        if start_row == current_row && col <= prompt_length {
            false
        } else {
            true
        }
    }
}

