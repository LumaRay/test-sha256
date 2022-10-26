// https://crates.io/crates/blake3
// 4692

use std::time::{SystemTime};//, UNIX_EPOCH};

const DATA_SIZE: usize = 1024 * 1024;
const LOOPS_COUNT: u32 = 16 * 1024;

fn main() {
    let data = [0u8; DATA_SIZE];
    let mut _hash = blake3::hash(&data);

    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        _hash = blake3::hash(&data);
    }
    println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}
