use crate::hash::{hash_with_report, get_hash_starter};
use crate::encrypt::three_fish_decrypt;
use base64::Engine;
use base64::engine::general_purpose;

pub fn key_decrypt(password: String, ciphertext: String, zero_height: u16, seed: u32) {
    let starter = get_hash_starter(password, seed);
    let hash = unsafe {  hash_with_report(starter, zero_height) };
    let key:Result<[u8; 64], _> = hash.1.as_slice().try_into();
    let key = match key {
        Ok(key) => key,
        Err(_) => panic!("Bug: key length is not 64 bytes")
    };
    let ciphertext_bytes = general_purpose::STANDARD_NO_PAD.decode(ciphertext.as_bytes()).unwrap();
    let plaintext_bytes = three_fish_decrypt(&key, ciphertext_bytes);
    let plaintext = String::from_utf8(plaintext_bytes).unwrap();
    println!("Plaintext: \n{}", plaintext);
}