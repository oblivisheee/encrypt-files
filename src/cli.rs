use crate::crypto::*;
use crate::file::*;
use crate::utils::read_line;
use std::path::Path;
use std::process::Command;
pub struct Interface;

impl Interface {
    pub fn run(&self) {
        loop {
            self.clear();
            println!("Welcome to encrypt-file. Enter 'encrypt' to encrypt a file or 'decrypt' to decrypt a file. To exit write 'exit'.");
            let answer = read_line(Some("Your choice: "));
            match answer.unwrap().as_str() {
                "encrypt" => self.encrypt_file(),
                "decrypt" => self.decrypt_file(),
                "exit" => break,
                _ => println!("Invalid choice"),
            }
        }
    }
    fn encrypt_file(&self) {
        self.clear();
        let file_path = read_line(Some("Write file path: ")).unwrap();
        if !Path::new(&file_path).exists() {
            println!("File not found.");
            return;
        }
        let key = generate_aes_key();
        let file_data = load_file(&file_path).unwrap();
        let encrypted = encrypt(&file_data, &key);
        save_file(&file_path, &encrypted).unwrap();
        self.clear();
        println!("Encrypted successfully.\n Your key: {}", hex::encode(&key));
        self.wait_for_enter()
    }
    fn decrypt_file(&self) {
        self.clear();
        let file_path = read_line(Some("Write file path: ")).unwrap();
        if !Path::new(&file_path).exists() {
            println!("File not found.");
            return;
        }
        let key = read_line(Some("Write key: ")).unwrap();
        let key_bytes = hex::decode(key).unwrap();
        let file_data = load_file(&file_path).unwrap();
        let decrypted = decrypt(&file_data, &key_bytes);
        save_file(&file_path, &decrypted).unwrap();
        self.clear();
        println!("Decrypted successfully.");
        self.wait_for_enter()
    }
    fn clear(&self) {
        let _ = Command::new("clear").status();
    }
    fn wait_for_enter(&self) {
        let _ = read_line(Some("To continue press enter."));
    }
}
