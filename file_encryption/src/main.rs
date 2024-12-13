mod cli;
mod encryption;
mod storage;

use cli::get_matches;
use encryption::{encrypt_file, decrypt_file};
use storage::{save_encrypted_file, save_key, load_key, read_encrypted_file};

fn main() {
    sodiumoxide::init().expect("Failed to initialize sodiumoxide");

    let matches = get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("File name is required");
            let content = std::fs::read(file_name).expect("Failed to read file");
    
            let key = sodiumoxide::crypto::secretbox::gen_key();
            save_key(&key);
    
            let encrypted_content = encrypt_file(&key, &content);
            save_encrypted_file(file_name, &encrypted_content);
    
            println!("File encrypted successfully: {}", file_name);
        }
        Some(("decrypt", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("File name is required");
            let key = load_key();
            let encrypted_content = read_encrypted_file(file_name);
    
            if let Some(decrypted_content) = decrypt_file(&key, &encrypted_content) {
                let output_file = format!("decrypted_{}", file_name);
                std::fs::write(&output_file, &decrypted_content)
                    .expect("Failed to save decrypted file");
                println!("File decrypted successfully: {}", output_file);
            } else {
                println!("Failed to decrypt the file.");
            }
        }
        _ => {
            eprintln!("Invalid command. Use --help for usage information.");
        }
    }   
}