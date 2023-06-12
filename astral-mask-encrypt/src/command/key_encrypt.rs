use base64::Engine;
use base64::engine::general_purpose;
use crate::hash::{get_hash_starter, hash_with_report};
use crate::encrypt::three_fish_encrypt;

pub fn key_encrypt(plaintext: String, password: String, seed: u32, zero_height: u16) {
    let password_bytes = get_hash_starter(password, seed);
    let hash = unsafe { hash_with_report(password_bytes, zero_height) };
    let key:Result<[u8; 64], _> = hash.1.as_slice().try_into();
    let key = match key {
        Ok(key) => key,
        Err(_) => panic!("Bug: key length is not 64 bytes")
    };
    let plaintext_bytes = three_fish_encrypt(key, plaintext.as_bytes().to_vec());
    let ciphertext = general_purpose::STANDARD_NO_PAD.encode(&plaintext_bytes);
    println!("Ciphertext: \n{}", ciphertext);
}