use std::fs;
use std::io::{BufRead, BufReader};

use std::ops::Add;

pub fn build(source_file: &'static str) -> Result<(), String> {
    let reader = {
        let file = fs::File::open(source_file);
        if file.is_err() {
            return Err("Unknown file error".to_string());
        }
        let file = file.unwrap();
        BufReader::new(file)
    };
    let mut html_doc = String::new(); 
    for (index, line) in reader.lines().enumerate() {
        let mut line = line.unwrap();
        if line.starts_with('#') {
            // heading
            let mut level: u8 = 0;
            while line.starts_with('#') {
                if level == 6 { break; }
                level += 1;
                line.remove(0);
            }
            if line.starts_with(' ') { line.remove(0); }
            let tag = format!("<h{}>{}</h{}>", level, line, level);
            html_doc = html_doc.add(tag.as_str());
        }
        println!("{}. {}", index, line);
    }
    return Ok(());
}

#[inline]
fn process_minor(line: &mut String) {
}

// pub fn validate()
