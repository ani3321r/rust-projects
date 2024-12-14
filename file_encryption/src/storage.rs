use sodiumoxide::crypto::secretbox::{seal, open, Key, Nonce};
use sodiumoxide::randombytes::randombytes;
use std::fs;

const NONCE_SIZE: usize = 24;

pub fn encrypt_file(key: &Key, content: &[u8]) -> Vec<u8> {
    // Generate a random nonce
    let nonce_bytes = randombytes(NONCE_SIZE);
    let nonce = Nonce::from_slice(&nonce_bytes).expect("Failed to generate nonce");

    let mut encrypted = nonce.0.to_vec();
    encrypted.extend(seal(content, &nonce, key));
    encrypted
}

/// Decrypt the encrypted file content using the provided key.
/// Returns `None` if decryption fails.
pub fn decrypt_file(key: &Key, encrypted_content: &[u8]) -> Option<Vec<u8>> {
    if encrypted_content.len() < NONCE_SIZE {
        return None;
    }

    let nonce = Nonce::from_slice(&encrypted_content[..NONCE_SIZE])
        .expect("Failed to extract nonce");
    let encrypted_data = &encrypted_content[NONCE_SIZE..];

    open(encrypted_data, &nonce, key).ok()
}

pub fn save_key(key: &Key) {
    fs::create_dir_all("keys").expect("Failed to create keys directory");
    fs::write("keys/key", key.0.as_ref()).expect("Failed to save key");
}

pub fn load_key() -> Key {
    let key_bytes = fs::read("keys/key").expect("Failed to read key. Ensure key file exists.");
    Key::from_slice(&key_bytes).expect("Invalid key format")
}

pub fn save_encrypted_file(file_name: &str, content: &[u8]) {
    fs::write(file_name, content).expect("Failed to save encrypted file");
}

pub fn read_encrypted_file(file_name: &str) -> Vec<u8> {
    fs::read(file_name).expect("Failed to read encrypted file")
}