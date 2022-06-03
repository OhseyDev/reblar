use std::vec;

type CmdFn = fn(&vec::Vec<String>) -> Result<(), String>;

const COMMANDS: [Option<CmdFn>; 26] = [
    None, Some(build), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None
];

pub fn process_command(c: char, args: &vec::Vec<String>) -> Result<(), String> {
    if c < 'a' || c > 'z' {
        return Err("Invalid command".to_string());
    }
    let cmd_num = (c as u8) - ('a' as u8);
    let cmd: Option<CmdFn> = COMMANDS[cmd_num as usize];
    match cmd {
        Some(fun) => { return fun(args); },
        None => { return Err("Invalid command".to_string()) }
    }
}

pub fn build(_args: &vec::Vec<String>) -> Result<(), String> {
    Ok(())
}