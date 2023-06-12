use threefish::Threefish512;
use byteorder::{ByteOrder, LittleEndian};

const DEFAULT_TWEAK: [u8; 16] = [0; 16];

pub fn three_fish_encrypt(key: [u8; 64], plaintext: Vec<u8>) -> Vec<u8> {
    // Create a new Three-fish-512 instance
    let cipher = Threefish512::new_with_tweak(&key, &DEFAULT_TWEAK);

    let mut ciphertext = Vec::new();

    // Convert plaintext bytes to u64 blocks and encrypt
    for chunk in plaintext.chunks(64) {
        let mut block = [0u64; 8];
        LittleEndian::read_u64_into(chunk, &mut block);

        cipher.encrypt_block_u64(&mut block);

        let mut encrypted_bytes = [0u8; 64];
        LittleEndian::write_u64_into(&block, &mut encrypted_bytes);
        ciphertext.extend_from_slice(&encrypted_bytes);
    }

    return ciphertext;
}

pub fn three_fish_decrypt(key: &[u8; 64], ciphertext: Vec<u8>) -> Vec<u8> {
    let cipher = Threefish512::new_with_tweak(key, &DEFAULT_TWEAK);

    // Ensure the ciphertext length is a multiple of 64, the block size for Three-fish-512.
    assert_eq!(ciphertext.len() % 64, 0);

    let mut blocks: Vec<[u64; 8]> = vec![Default::default(); ciphertext.len() / 64];

    // Convert ciphertext bytes to blocks of u64
    for (i, block) in blocks.iter_mut().enumerate() {
        for j in 0..8 {
            let start = i * 64 + j * 8;
            let end = start + 8;
            let bytes = &ciphertext[start..end];
            block[j] = u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
                bytes[4], bytes[5], bytes[6], bytes[7],
            ]);
        }
    }

    // Decrypt each block
    for block in &mut blocks {
        cipher.decrypt_block_u64(block);
    }

    // Convert decrypted blocks back to bytes
    let mut plaintext = Vec::new();
    for block in blocks {
        for &value in block.iter() {
            plaintext.extend_from_slice(&value.to_le_bytes());
        }
    }

    return plaintext;
}