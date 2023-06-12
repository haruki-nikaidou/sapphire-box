use crate::hash::{get_hash_starter, hash_with_report};
use base64::Engine;
use base64::engine::general_purpose;

pub fn mint_key (password: String, seed: u32, zero_height: u16) {
    let password_bytes = get_hash_starter(password, seed);
    let hash = unsafe { hash_with_report(password_bytes, zero_height) };
    let hash_text = general_purpose::STANDARD_NO_PAD.encode(&hash.1);
    println!("Finished Minting Key with {} circles", hash.0);
    println!("Key: \n {}", hash_text);
}