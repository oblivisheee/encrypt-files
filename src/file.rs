use std::fs;

pub fn load_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(file_path)
}
pub fn save_file(file_path: &str, contents: &[u8]) -> Result<(), std::io::Error> {
    fs::write(file_path, contents)
}
