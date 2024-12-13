use sodiumoxide::crypto::secretbox;

pub fn encrypt_file(key: &secretbox::Key, plaintext: &[u8]) -> Vec<u8> {
    let nonce = secretbox::gen_nonce();
    let ciphertext = secretbox::seal(plaintext, &nonce, key);

    println!("Key: {:?}", key.0);
    println!("Nonce: {:?}", nonce.0);
    println!("Ciphertext: {:?}", ciphertext);

    [nonce.0.to_vec(), ciphertext].concat()
}

pub fn decrypt_file(key: &secretbox::Key, encrypted_content: &[u8]) -> Option<Vec<u8>> {
    let (nonce, ciphertext) = encrypted_content.split_at(secretbox::NONCEBYTES);

    println!("Key: {:?}", key.0);
    println!("Nonce: {:?}", nonce);
    println!("Ciphertext: {:?}", ciphertext);

    let nonce = secretbox::Nonce::from_slice(nonce).expect("Invalid nonce format");
    secretbox::open(ciphertext, &nonce, key).ok()
}