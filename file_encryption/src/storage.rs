use sodiumoxide::crypto::secretbox;

pub fn save_encrypted_file(file_name: &str, content: &[u8]) {
    let path = format!("encrypted_files/{}", file_name);
    std::fs::write(&path, content).expect("Failed to save encrypted file");
}

pub fn read_encrypted_file(file_name: &str) -> Vec<u8> {
    let path = format!("encrypted_files/{}", file_name);
    std::fs::read(&path).expect("Failed to read encrypted file")
}

pub fn save_key(key: &secretbox::Key) {
    let path = "keys/encryption.key";
    std::fs::write(path, key.0).expect("Failed to save key");
}

pub fn load_key() -> secretbox::Key {
    let data = std::fs::read("keys/encryption.key").expect("Failed to read key");
    secretbox::Key::from_slice(&data).expect("Invalid key format")
}