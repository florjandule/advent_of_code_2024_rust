use std::fs::OpenOptions;
use std::io::{self, Read};

pub fn read_input(file_path: &str) -> io::Result<String> {
    let mut file = OpenOptions::new()
    .read(true)
    .open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_filename_without_extension(path: &str) -> Option<String> {
    let path = std::path::Path::new(path);
    path.file_stem()
        .and_then(|stem| stem.to_str().map(|s| s.to_string()))
}