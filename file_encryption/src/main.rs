use clap::Arg;
use sodiumoxide::crypto::secretbox::{gen_key, gen_nonce, Key, Nonce};
use std::fs;

mod cli;
mod storage;
use storage::{decrypt_file, encrypt_file, load_key, read_encrypted_file, save_encrypted_file, save_key};

fn main() {
    sodiumoxide::init().expect("Failed to initialize sodiumoxide");

    let matches = cli::get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("File name is required");
            let content = fs::read(file_name).expect("Failed to read file");

            let key = gen_key();
            save_key(&key);

            let encrypted_content = encrypt_file(&key, &content);
            let encrypted_file_name = format!("{}.enc", file_name);
            save_encrypted_file(&encrypted_file_name, &encrypted_content);

            println!("File encrypted successfully: {}", encrypted_file_name);
        }
        Some(("decrypt", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE").expect("File name is required");
            let key = load_key();
            let encrypted_content = read_encrypted_file(file_name);

            if let Some(decrypted_content) = decrypt_file(&key, &encrypted_content) {
                let output_file = file_name.strip_suffix(".enc").unwrap_or("decrypted_file");
                fs::write(output_file, &decrypted_content).expect("Failed to save decrypted file");
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
