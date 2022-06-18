use std::fs;
use std::io::{BufRead, BufReader};

pub fn build(source_file: &'static str) -> Result<(), String> {
    let reader = {
        let file = fs::File::open(source_file);
        if file.is_err() {
            return Err("Unknown file error".to_string());
        }
        let file = file.unwrap();
        BufReader::new(file)
    };
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // TODO: handle parsing of Markdown document
        println!("{}. {}", index, line);
    }
    return Ok(());
}

// pub fn validate()
