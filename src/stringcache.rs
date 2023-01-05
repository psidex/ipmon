use std::{fs, io};

pub fn get_str(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn set_str(path: &str, value: &str) -> io::Result<()> {
    fs::write(path, value)
}
