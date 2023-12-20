mod cli;
mod command;
mod core;

use clap::Parser;
use cli::{Cli, Commands};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use command::{decrypt, encrypt};

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
        Commands::Encrypt { plaintext, password, dial } => {
            encrypt::encrypt_cmd(plaintext, password, dial);
        },
        Commands::Decrypt { ciphertext, password, dial } => {
            decrypt::decrypt_cmd(ciphertext, password, dial);
        },
    }
}
