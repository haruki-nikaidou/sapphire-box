use crate::core::encode::{decode_utf8, encode_base64};
use crate::core::combination_lock::CombinationLock;
use crate::command::parse_array;

pub fn encrypt_cmd(plain_text: String, key: String, dial: String) {
    let plain_text = decode_utf8(plain_text);
    let key = decode_utf8(key);
    let dial = parse_array::string_to_vec(dial);
    let mut lock = CombinationLock::new(key, dial);
    let cipher_text = lock.encrypt(&plain_text);
    let result = encode_base64(cipher_text);
    println!("Encrypted text:\n {}", result);
}