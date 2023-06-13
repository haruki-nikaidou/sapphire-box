use base64::Engine;
use base64::engine::general_purpose;
use crate::encrypt::three_fish_encrypt;

pub fn hash_encrypt(plaintext: String, hash: String) {
    tracing::warn!("You should not encrypt your plaintext with a hash directly. That is not safe.");
    let key:Result<[u8; 64], _> = hash.as_bytes().to_vec().try_into();
    let key = match key {
        Ok(key) => key,
        Err(_) => panic!("Invalid Input: hash length is not 64 bytes")
    };
    let result = three_fish_encrypt(key, plaintext.as_bytes().to_vec());
    let ciphertext = general_purpose::STANDARD_NO_PAD.encode(&result);
    println!("Ciphertext: \n{}", ciphertext);
}