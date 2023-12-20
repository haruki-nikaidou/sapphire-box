use sha2::{Sha512, Digest};
use std::convert::TryInto;
use crate::core::pow_512;

pub fn get_hash_keys(start: &Vec<u8>, zeros: &Vec<i32>) -> Vec<[u8; 64]> {
    // init the first hash
    let mut hasher = Sha512::new();
    hasher.update(&start);
    let first_hash: [u8; 64] = hasher.finalize().try_into().unwrap();

    // init the results vector
    let mut results = Vec::new();

    // iterate over the zeros vector
    let mut previous_result: [u8;64] = first_hash;
    for zero in zeros {
        let result = pow_512(previous_result, zero.clone());
        results.push(result);
        previous_result = result;
    }

    return results;
}