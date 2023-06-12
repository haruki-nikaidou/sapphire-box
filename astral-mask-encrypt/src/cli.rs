use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, author)]
#[command(about = "Astral Mask Encrypt Cli", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  #[command(about = "Encrypt a string with a password and seed")]
  KeyEncrypt {
    #[arg(long, help = "The string to encrypt")]
    #[arg(required = true)]
    #[arg(value_name = "PLAINTEXT")]
    plaintext: String,

    #[arg(long, help = "The password to encrypt the file with")]
    #[arg(required = true)]
    #[arg(value_name = "PASSWORD")]
    password: String,

    #[arg(short, long, help = "The seed to encrypt the file with")]
    #[arg(value_name = "SEED", default_value = "128")]
    seed: u32,

    #[arg(short, long, help = "Encrypt difficulty")]
    #[arg(value_name = "DIFFICULTY", default_value = "24")]
    zero_height: u16,
  },

  #[command(about = "Encrypt a string with a hash")]
  HashEncrypt {
    #[arg(short, long, help = "The string to encrypt")]
    #[arg(required = true)]
    #[arg(value_name = "PLAINTEXT")]
    plaintext: String,

    #[arg(short, long, help = "The hash to encrypt the file with")]
    #[arg(required = true)]
    #[arg(value_name = "HASH")]
    hash: String
  },

  #[command(about = "Generate the hash which is used to encrypt a string. Never save this hash anywhere!")]
  MintKey {
    #[arg(short, long, help = "The password to generate the key with")]
    #[arg(required = true)]
    #[arg(value_name = "PASSWORD")]
    password: String,

    #[arg(short, long, help = "The seed to generate the key with")]
    #[arg(value_name = "SEED", default_value = "128")]
    seed: u32,

    #[arg(short, long, help = "Difficulty")]
    #[arg(value_name = "DIFFICULTY", default_value = "24")]
    zero_height: u16,
  },

  #[command(about = "Decrypt a string with a password and seed")]
  KeyDecrypt {
    #[arg(short, long, help = "The string to decrypt")]
    #[arg(required = true)]
    #[arg(value_name = "CIPHERTEXT")]
    ciphertext: String,

    #[arg(short, long, help = "The password to decrypt the file with")]
    #[arg(required = true)]
    #[arg(value_name = "PASSWORD")]
    password: String,

    #[arg(short, long, help = "The seed to decrypt the file with")]
    #[arg(value_name = "SEED", default_value = "128")]
    seed: u32,

    #[arg(short, long, help = "Encrypt difficulty")]
    #[arg(value_name = "DIFFICULTY", default_value = "24")]
    zero_height: u16,
  },

  #[command(about = "Decrypt a string with a hash")]
  HashDecrypt {
    #[arg(short, long, help = "The string to decrypt")]
    #[arg(required = true)]
    #[arg(value_name = "CIPHERTEXT")]
    ciphertext: String,

    #[arg(short, long, help = "The hash to decrypt the file with")]
    #[arg(required = true)]
    #[arg(value_name = "HASH")]
    hash: String
  },
}