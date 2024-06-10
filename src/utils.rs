use std::fs;
use std::io::{self, Write};

pub fn read_line(prefix: Option<&str>) -> io::Result<String> {
    if let Some(prefix) = prefix {
        print!("{}", prefix);
    }
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_string())
}

pub fn load_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(file_path)
}
pub fn save_file(file_path: &str, contents: &[u8]) -> Result<(), std::io::Error> {
    fs::write(file_path, contents)
}
