use crate::core::get_keys::get_hash_keys;
use crate::core::three_fish;

/// # Combination Lock
/// Encrypting data using a data structure similar to the combination lock dial of a safe deposit box.
/// To make a combination lock, you need a `key` and a `dial`. The key is a series of bytes, and the dial is a series of (positive of negative) integers.
pub struct CombinationLock {
    pub key: Vec<u8>,
    pub dial: Vec<i32>,
    pub hash_series: Option<Vec<[u8; 64]>>,
}

impl CombinationLock {
    pub(crate) fn new(key: Vec<u8>, dial: Vec<i32>) -> CombinationLock {
        CombinationLock {
            key,
            dial,
            hash_series: None,
        }
    }
    fn generate_hash_series(&mut self) {
        self.hash_series = Some(get_hash_keys(&self.key, &self.dial));
    }
    /// Encrypts a vector of bytes using the combination lock.
    pub fn encrypt(&mut self, plain_text: &Vec<u8>) -> Vec<u8> {
        let key_series = match self.hash_series.clone() {
            Some(series) => series,
            None => {
                self.generate_hash_series();
                self.hash_series.clone().unwrap()
            },
        };
        let mut cipher_text = plain_text.clone();
        for key in key_series {
            cipher_text = three_fish::encrypt(&cipher_text, &key);
        }
        return cipher_text;
    }
    /// Decrypts a vector of bytes which was encrypted using the combination lock.
    pub fn decrypt(&mut self, cipher_text: &Vec<u8>) -> Vec<u8> {
        let key_series = match self.hash_series.clone() {
            Some(series) => series,
            None => {
                self.generate_hash_series();
                self.hash_series.clone().unwrap()
            },
        };
        let mut plain_text = cipher_text.clone();
        let reverse = key_series.iter().rev();
        for key in reverse {
            plain_text = three_fish::decrypt(&key, cipher_text);
        }
        return plain_text;
    }
}