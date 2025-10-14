use std::fs;
use std::io;

pub fn calculate() -> io::Result<String> {
    let data = std::fs::read_to_string("exampleInput.txt")?;
    Ok(data)
}