use crate::core::encode::{decode_base64, encode_utf8};
use crate::core::combination_lock::CombinationLock;
use crate::command::parse_array;

pub fn decrypt_cmd(cipher_text: String, key: String, dial: String) {
    let cipher_text = decode_base64(cipher_text);
    let key = decode_base64(key);
    let dial = parse_array::string_to_vec(dial);
    let mut lock = CombinationLock::new(key, dial);
    let plain_text = lock.decrypt(&cipher_text);
    let result = encode_utf8(plain_text);
    println!("Decrypted text:\n {}", result);
}