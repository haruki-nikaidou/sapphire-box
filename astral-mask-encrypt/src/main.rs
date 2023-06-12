mod encrypt;
mod hash;
mod cli;
mod command;


use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::HashEncrypt { plaintext, hash } => {
            command::hash_encrypt(plaintext, hash);
        },
        Commands::KeyEncrypt { plaintext, password, seed, zero_height } => {
            command::key_encrypt(plaintext, password, seed, zero_height);
        },
        Commands::MintKey { password, seed, zero_height } => {
            command::mint_key(password, seed, zero_height);
        },
        Commands::KeyDecrypt { password, ciphertext, seed, zero_height } => {
            command::key_decrypt(password, ciphertext, zero_height, seed);
        },
        Commands::HashDecrypt { ciphertext, hash } => {
            command::hash_decrypt(ciphertext, hash);
        }
    }
}
