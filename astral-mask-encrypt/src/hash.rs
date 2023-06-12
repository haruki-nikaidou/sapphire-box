extern crate sha2;
use sha2::{Sha512, Digest};
use std::cell::UnsafeCell;
use std::thread;
use std::sync::{Arc, mpsc};
use std::time::Duration;

/// This function takes a `start` value and a `zero_height` value. It then hashes the `start` value until the first `zero_height` bits of the hash are zero.
/// It returns the number of cycles and hash result as `(loops_circle, hash_result)`.
#[allow(dead_code)]
pub fn mint_key_512(start: Vec<u8>, zero_height: u16) -> (u64, Vec<u8>) {
    let mut hasher = Sha512::new();
    let mut iterations = 0;

    loop {
        // Increment the iteration count
        iterations += 1;

        // Hash the input data
        hasher.update(&start);
        let result = hasher.finalize_reset();

        // Check if the first `zero_height` bits of the hash are zero
        if check_zero_bits(&result, zero_height) {
            return (iterations, result.to_vec());
        }
    }
}

pub struct Monitor {
    counter: UnsafeCell<u64>,
}
impl Monitor {
    pub fn new() -> Self {
        Self {
            counter: UnsafeCell::new(0),
        }
    }
    pub fn get_number(&self) -> u64 {
        unsafe { *self.counter.get().clone() }
    }
    pub fn plus_one(&self) {
        unsafe {
            *self.counter.get() += 1;
        }
    }
}
unsafe impl Sync for Monitor {}

pub unsafe fn async_mint_key_512 (monitor: Arc<Monitor>, start: Vec<u8>, zero_height: u16) -> (u64, Vec<u8>) {
    let mut hasher = Sha512::new();

    loop {
        // Increment the iteration count
        monitor.plus_one();

        // Hash the input data
        hasher.update(&start);
        let result = hasher.finalize_reset();

        // Check if the first `zero_height` bits of the hash are zero
        if check_zero_bits(&result, zero_height) {
            return (monitor.get_number(), result.to_vec());
        }
    }
}


pub unsafe fn hash_with_report(start: Vec<u8>, zero_height: u16) -> (u64, Vec<u8>) {
    let (sender, receiver) = mpsc::channel();
    let mut completed_hash_circle = 0;
    let monitor = Arc::new(Monitor::new());
    let monitor_clone = Arc::clone(&monitor);

    thread::spawn(move || {
        unsafe {
            let result = async_mint_key_512(monitor_clone, start, zero_height);
            sender.send(result).unwrap();
        }
    });

    loop {
        thread::sleep(Duration::from_secs(20));
        let circle_in_this_20s = monitor.get_number() - completed_hash_circle;
        let million_circle_per_sec = (circle_in_this_20s as f64) / (20.0 * 1e6 as f64);
        match receiver.try_recv() {
            Ok(result) => return result,
            Err(_) => {
                println!("Minting Key: {} MH/s", million_circle_per_sec);
                completed_hash_circle = monitor.get_number();
            }
        }
    }
}

pub fn get_hash_starter(password: String, repeat: u32) -> Vec<u8> {
    let mut password_bytes = password.into_bytes();
    for _ in 0..repeat {
        let clone = password_bytes.clone();
        password_bytes.extend_from_slice(&clone);
    }
    return password_bytes;
}

fn check_zero_bits(hash: &[u8], zero_height: u16) -> bool {
    let zero_bytes = (zero_height / 8) as usize;
    let zero_bits = (zero_height % 8) as usize;

    // Check if the first `zero_bytes` bytes are zero
    if !hash.iter().take(zero_bytes).all(|&byte| byte == 0) {
        return false;
    }

    // Check if the next `zero_bits` bits are zero
    if zero_bits > 0 && hash[zero_bytes] >> (8 - zero_bits) != 0 {
        return false;
    }

    true
}
