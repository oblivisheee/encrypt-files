use rand::Rng;
use soft_aes::aes::{aes_dec_ecb, aes_enc_ecb};
use zeroize::Zeroizing;

pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    aes_enc_ecb(plaintext, key, Some("PKCS7")).unwrap()
}

pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    aes_dec_ecb(ciphertext, key, Some("PKCS7")).unwrap()
}

pub fn generate_aes_key() -> Zeroizing<[u8; 32]> {
    let mut key = [0u8; 32];
    rand::thread_rng().fill(&mut key);
    Zeroizing::new(key)
}
