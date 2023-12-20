const BLOCK_SIZE: usize = 64;

pub fn pad(plaintext: &mut Vec<u8>) {
    let padding_value = (BLOCK_SIZE - (plaintext.len() % BLOCK_SIZE)) as u8;

    let padding = vec![padding_value; padding_value as usize];
    plaintext.extend_from_slice(&padding);
}

pub fn unpad(plaintext: &mut Vec<u8>) {
    if let Some(&last_byte) = plaintext.last() {
        let padding_value = last_byte as usize;

        if padding_value <= plaintext.len() {
            plaintext.truncate(plaintext.len() - padding_value);
        }
    }
}