use clap::{Arg, Command};

pub fn get_matches() -> clap::ArgMatches {
    Command::new("Encrypted Storage")
        .version("1.0")
        .about("Encrypt and decrypt files securely")
        .subcommand(
            Command::new("encrypt")
                .about("Encrypt a file")
                .arg(
                    Arg::new("FILE")
                        .help("File to encrypt")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt a file")
                .arg(
                    Arg::new("FILE")
                        .help("File to decrypt")
                        .required(true),
                ),
        )
        .get_matches()
}
