mod encrypt;
mod hash;
mod cli;
mod command;

use clap::Parser;
use cli::{Cli, Commands};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn setup_tracing() {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .with_target(false)
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn main() {
    setup_tracing();
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
