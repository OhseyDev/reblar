use std::fs;

pub fn build(source_file: &'static str) -> Result<(), String> {
    let file = fs::File::open(source_file);
    if file.is_err() {
        return Err("Unknown file error".to_string());
    }
    return Ok(());
}

// pub fn validate()
