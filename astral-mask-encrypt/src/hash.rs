use sha2::{Sha512, Digest};
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::thread;
use std::sync::{Arc, mpsc};
use std::time::Duration;

/// This function takes a `start` value and a `zero_height` value. It then hashes the `start` value until the first `zero_height` bits of the hash are zero.
/// It returns the number of cycles and hash result as `(loops_circle, hash_result)`.
#[allow(dead_code)]
pub fn mint_key_512(start: Vec<u8>, zero_height: u16) -> (u64, Vec<u8>) {
    let mut iterations = 0;
    let mut hasher = Sha512::new();
    hasher.update(&start);
    let mut binding = hasher.finalize_reset();
    let mut result = binding.as_slice();

    loop {
        // Check if the first `zero_height` bits of the hash are zero
        if check_zero_bits(result, zero_height) {
            return (iterations, result.to_vec());
        }

        // Hash the input data
        hasher.update(&result);
        binding = hasher.finalize_reset();
        result = binding.as_slice();

        // Increment the iteration count
        iterations += 1;
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

pub unsafe fn async_mint_key_512(monitor: Arc<Monitor>, start: Vec<u8>, zero_height: u16) -> (u64, Vec<u8>) {
    let monitor = monitor.deref();
    let mut hasher = Sha512::new();
    hasher.update(&start);
    let mut binding = hasher.finalize_reset();
    let mut result = binding.as_slice();

    loop {
        // Check if the first `zero_height` bits of the hash are zero
        if check_zero_bits(result, zero_height) {
            return (monitor.get_number(), result.to_vec());
        }

        // Hash the input data
        hasher.update(&result);
        binding = hasher.finalize_reset();
        result = binding.as_slice();

        // Increment the iteration count
        monitor.plus_one();
    }
}


pub unsafe fn hash_with_report(start: Vec<u8>, zero_height: u16) -> (u64, Vec<u8>) {
    let (hash_sender, hash_receiver) = mpsc::channel();
    let (counter_stop_sender, counter_stop_receiver) = mpsc::channel();
    let mut completed_hash_circle = 0;
    let monitor = Arc::new(Monitor::new());
    let monitor_clone1 = Arc::clone(&monitor);
    let monitor_clone2 = Arc::clone(&monitor);

    // async function
    thread::spawn(move || {
        let result = unsafe { async_mint_key_512(monitor_clone1, start, zero_height) };
        match hash_sender.send(result) {
            Err(_) => {
                tracing::error!("Bug: unable to send the result in hash_with_report function");
                panic!("Bug: unable to send the result in hash_with_report function");
            }
            _ => {}
        };
    });

    // report
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));
            let circle_in_this_20s = monitor_clone2.get_number() - completed_hash_circle;
            let kilo_circle_per_sec = (circle_in_this_20s as f64) / (10.0 * 1e3 as f64);
            match counter_stop_receiver.try_recv() {
                Ok(_) => break,
                Err(_) => {
                    tracing::info!("Minting Key: {} KH/s", kilo_circle_per_sec);
                    completed_hash_circle = monitor_clone2.get_number();
                }
            }
        }
    });

    // wait for the result
    loop {
        thread::sleep(Duration::from_secs(1));
        match hash_receiver.try_recv() {
            Ok(result) => {
                counter_stop_sender.send(()).unwrap();
                return result;
            }
            Err(_) => {}
        }
    }
}

pub fn get_hash_starter(password: String, repeat: u32) -> Vec<u8> {
    let cell = password.into_bytes();
    let mut password_bytes = cell.clone();
    for _ in 0..repeat {
        password_bytes.extend_from_slice(&cell);
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

    return true;
}
