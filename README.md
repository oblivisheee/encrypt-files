### Description

This app implements a tool to encrypt and decrypt files via a simple CLI.

To encrypt a file, ensure you have Rust installed and execute the following commands:

MacOS, Linux, and Windows PowerShell:

```
git clone https://github.com/oblivisheee/encrypt-files.git
cd encrypt-files
cargo run
```

### Advanced Description

This app uses AES256 to encrypt and decrypt files. The encryption and decryption functionality is implemented in the `src/crypto.rs` file.

The `encrypt` function takes a plaintext byte slice and a key byte slice as input, and returns a vector of bytes representing the encrypted ciphertext. It uses the `aes_enc_ecb` function from the `soft_aes` crate to perform AES encryption in ECB mode with PKCS7 padding.

The `decrypt` function takes a ciphertext byte slice and a key byte slice as input, and returns a vector of bytes representing the decrypted plaintext. It uses the `aes_dec_ecb` function from the `soft_aes` crate to perform AES decryption in ECB mode with PKCS7 padding.

The `generate_aes_key` function generates a random 32-byte key suitable for use with AES256. It uses the `rand` crate to generate a cryptographically secure random key.
