use crate::encrypt::three_fish_decrypt;
use base64::Engine;
use base64::engine::general_purpose;

pub fn hash_decrypt(hash: String, ciphertext: String) {
    tracing::warn!("You should never save your hash key! If you do, you are at risk of being hacked!");
    let hash_bytes = general_purpose::STANDARD_NO_PAD.decode(hash.as_bytes()).unwrap();
    let key:Result<[u8; 64], _> = hash_bytes.as_slice().try_into();
    let key = match key {
        Ok(key) => key,
        Err(_) => panic!("Bug: key length is not 64 bytes")
    };
    let ciphertext_bytes = general_purpose::STANDARD_NO_PAD.decode(ciphertext.as_bytes()).unwrap();
    let plaintext_bytes = three_fish_decrypt(&key, ciphertext_bytes);
    let plaintext = String::from_utf8(plaintext_bytes).unwrap();
    println!("Plaintext: \n{}", plaintext);
}