use crate::crypto::*;
use crate::utils::{load_file, read_line, save_file};
use std::path::Path;
use std::process::Command;
use zeroize::{Zeroize, Zeroizing};

pub struct Interface;

impl Interface {
    pub fn run(&self) {
        loop {
            self.clear();
            println!("Welcome to encrypt-file. Enter 'encrypt' to encrypt a file or 'decrypt' to decrypt a file. Advanced mode available for Pro users, activate via 'advanced'. To exit write 'exit'.");
            let answer = read_line(Some("Your choice: ")).unwrap();
            match answer.as_str() {
                "encrypt" => self.encrypt_file(),
                "decrypt" => self.decrypt_file(),
                "advanced" => self.advanced_mode(),
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
            self.wait_for_enter();
            return;
        }

        let mut key = generate_aes_key();
        let file_data = load_file(&file_path).unwrap();
        let encrypted = encrypt(&file_data, &key.as_slice());
        if let Err(e) = save_file(&file_path, &encrypted) {
            println!("Error saving encrypted file: {}", e);
            self.wait_for_enter();
            return;
        }

        self.clear();
        println!("Encrypted successfully.\n Your key: {}", hex::encode(&key));
        key.zeroize();
        self.wait_for_enter()
    }

    fn decrypt_file(&self) {
        self.clear();
        let file_path = read_line(Some("Write file path: ")).unwrap();
        if !Path::new(&file_path).exists() {
            println!("File not found.");
            self.wait_for_enter();
            return;
        }

        let mut key = read_line(Some("Write key: ")).unwrap();
        if key.len() != 64 {
            println!("Invalid key length. Key must be 64 characters long.");
            self.wait_for_enter();
            return;
        }

        let key_bytes = Zeroizing::new(hex::decode(key.clone()).unwrap());
        key.zeroize();
        let file_data = load_file(&file_path).unwrap();
        let decrypted = decrypt(&file_data, &key_bytes);
        if let Err(e) = save_file(&file_path, &decrypted) {
            println!("Error saving decrypted file: {}", e);
            self.wait_for_enter();
            return;
        }

        self.clear();
        println!("Decrypted successfully.");
        self.wait_for_enter()
    }

    fn advanced_mode(&self) {
        loop {
            self.clear();
            println!("Welcome to the advanced mode. Type 'return' to return to the main menu. Type 'encrypt' to encrypt a file or 'decrypt' to decrypt a file.");
            let answer = read_line(Some("Your choice: ")).unwrap();
            match answer.as_str() {
                "return" => break,
                "encrypt" => self.advanced_encrypt(),
                "decrypt" => self.advanced_decrypt(),
                _ => {
                    println!("Invalid choice");
                    self.wait_for_enter();
                }
            }
        }
    }

    fn advanced_encrypt(&self) {
        self.clear();
        let file_path = read_line(Some("Write file path: ")).unwrap();
        if !Path::new(&file_path).exists() {
            println!("File not found.");
            self.wait_for_enter();
            return;
        }

        let mut key = read_line(Some("Write key (leave empty to skip): ")).unwrap();
        let mut key_bytes: Zeroizing<[u8; 32]> = if key.is_empty() {
            let key_bytes = generate_aes_key();
            println!("Your Key: {}", hex::encode(key_bytes.clone()));
            key_bytes
        } else {
            Zeroizing::new(hex::decode(&key).unwrap().try_into().unwrap())
        };
        key.zeroize();

        let file_data = load_file(&file_path).unwrap();
        let encrypted = encrypt(&file_data, &key_bytes.as_slice());
        key_bytes.zeroize();

        let file_name_answer = read_line(Some("Write file name(leave empty to skip): ")).unwrap();
        let file_name: &str = if file_name_answer.is_empty() {
            &file_path
        } else {
            &file_name_answer
        };

        if let Err(e) = save_file(file_name, &encrypted) {
            println!("Error saving encrypted file: {}", e);
            self.wait_for_enter();
            return;
        }

        self.clear();
        println!("Encrypted successfully.");
        self.wait_for_enter()
    }

    fn advanced_decrypt(&self) {
        self.clear();
        let file_path = read_line(Some("Write file path: ")).unwrap();
        if !Path::new(&file_path).exists() {
            println!("File not found.");
            self.wait_for_enter();
            return;
        }

        let mut key = read_line(Some("Write key (leave empty to skip): ")).unwrap();
        let mut key_bytes: Zeroizing<[u8; 32]> = if key.is_empty() {
            let key_bytes = generate_aes_key();
            println!("Your key: {}", hex::encode(&key_bytes));
            key_bytes
        } else {
            Zeroizing::new(hex::decode(&key).unwrap().try_into().unwrap())
        };
        key.zeroize();

        let file_data = load_file(&file_path).unwrap();
        let decrypted = decrypt(&file_data, &key_bytes.as_slice());
        key_bytes.zeroize();

        let file_name_answer = read_line(Some("Write file name(leave empty to skip): ")).unwrap();
        let file_name: &str = if file_name_answer.is_empty() {
            &file_path
        } else {
            &file_name_answer
        };

        if let Err(e) = save_file(file_name, &decrypted) {
            println!("Error saving decrypted file: {}", e);
            self.wait_for_enter();
            return;
        }

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
