use std::vec;
use std::fs;

type CmdFn = fn(&vec::Vec<String>) -> Result<(), String>;

const COMMANDS: [Option<CmdFn>; 26] = [
    None, Some(build_all), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None
];

/*
    a => (reserved)
    b => build all
    c => compile all
    d => distribute
    e => build directory
    f => compile file
    g => bundle directory
    h => bundle files
    i => index all
    j => distribute bundle
    k...z => reserved
*/
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

fn build_all(_args: &vec::Vec<String>) -> Result<(), String> {
    let dir = {
        let mut path = dirs::home_dir().unwrap();
        path.push("src");
        let dir = fs::read_dir(path.as_path());
        if dir.is_err() { return Err("unable to read directory".to_string()); }
        dir.unwrap()
    };
    for path in dir {
        let path = path.unwrap();
        let _metadata = path.metadata().unwrap();
        // TODO: Finish implementation
    }
    return Ok(());
}
