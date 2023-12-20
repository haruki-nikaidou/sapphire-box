use std::cell::UnsafeCell;
use sha2::{Sha512, Digest};
use std::convert::TryInto;
use std::sync::Arc;

/// check whether the first `zero_height` bits of the hash are zero
fn check_zero_head(bytes: &[u8], zero_head: u32) -> bool {
    for i in 0..zero_head {
        let byte = bytes[(i / 8) as usize];
        let bit = (byte >> (7 - (i % 8))) & 1;
        if bit != 0 {
            return false;
        }
    }
    true
}

/// check whether the last `zero_height` bits of the hash are zero
fn check_zero_tail(bytes: &[u8], zero_tail: u32) -> bool {
    for i in 0..zero_tail {
        let byte = bytes[bytes.len() - 1 - (i / 8) as usize];
        let bit = (byte >> (i % 8)) & 1;
        if bit != 0 {
            return false;
        }
    }
    true
}

/// ### A Counter that count the loop count of the pow_512 function
/// Sometimes, `pow_512` function spends a lot of time to find a hash.
/// If no monitor, we can't know whether the program is still running or it is dead.
pub struct WorkMonitor {
    finished: UnsafeCell<u64>
}

impl WorkMonitor {
    fn new() -> Self {
        WorkMonitor {
            finished: UnsafeCell::new(0)
        }
    }
    #[allow(dead_code)]
    fn get(&self) -> u64 {
        unsafe { *self.finished.get() }
    }
    fn add_one(&self) {
        unsafe { *self.finished.get() += 1 }
    }
}

/// ### The `pow_512` function with monitor
pub fn pow_512_with_monitor(start: [u8; 64], zero_length: i32, monitor: Arc<WorkMonitor>) -> [u8; 64] {
        let mut buffer = start;

    if zero_length > 512 || zero_length < -512 {
        panic!("zero_length must be in range [-512, 512]");
    }

    let head_or_tail = zero_length > 0; // true: head, false: tail
    let zero_length = zero_length.abs() as u32;

    loop {
        monitor.add_one();
        // create a Sha512 object and init
        let mut hasher = Sha512::new();
        hasher.update(&buffer);

        // read hash digest and consume hasher
        let result = hasher.finalize();
        let bytes = &result.as_slice()[0..64];

        // check if the first or last `zero_length` bit are zero
        if head_or_tail && check_zero_head(&bytes, zero_length)
            || !head_or_tail && check_zero_tail(&bytes, zero_length) {
            return bytes.try_into().unwrap();
        }

        // if not zero, continue
        buffer = bytes[0..64].try_into().unwrap();
    }
}

/// ### The function to generate a key
/// Iterate for sha512 until the first or last `zero_length` bits are zero
/// when `zero_length` is positive, check the first `zero_length` bits
/// when `zero_length` is negative, check the last `zero_length` bits
pub fn pow_512(start: [u8; 64], zero_length: i32) -> [u8; 64] {
    // function `pow_512` is based on `pow_512_with_monitor`
    // but it doesn't have a monitor
    // so we create a useless monitor to call `pow_512_with_monitor`
    let useless_monitor = Arc::new(WorkMonitor::new());
    pow_512_with_monitor(start, zero_length, useless_monitor)
}