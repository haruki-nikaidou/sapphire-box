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
  #[command(about = "Encrypt a string with a Combination Lock")]
  Encrypt {
    #[arg(long, short, help = "The string to encrypt")]
    #[arg(required = true)]
    #[arg(value_name = "TEXT")]
    plaintext: String,

    #[arg(long, short, help = "The password to encrypt the String with")]
    #[arg(required = true)]
    #[arg(value_name = "PASSWORD")]
    password: String,

    #[arg(short, long, help = "The dial to encrypt the file with. Should be like [-1,2,8,-5,2]")]
    #[arg(required = true)]
    #[arg(value_name = "DIAL")]
    dial: String,
  },

  #[command(about = "Decrypt a string which was encrypted with a Combination Lock")]
  Decrypt {
    #[arg(short, long, help = "The string to decrypt")]
    #[arg(required = true)]
    #[arg(value_name = "CIPHERTEXT")]
    ciphertext: String,

    #[arg(long, short, help = "The password to encrypt the file with")]
    #[arg(required = true)]
    #[arg(value_name = "PASSWORD")]
    password: String,

    #[arg(short, long, help = "The dial to encrypt the file with. Should be like [-1,2,8,-5,2]")]
    #[arg(required = true)]
    #[arg(value_name = "DIAL")]
    dial: String,
  },
}