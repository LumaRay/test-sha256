// https://github.com/ActualIlluminati/sha256fast
// https://crates.io/crates/sha256fast
// 

use sha256fast::{Digest, Sha256};
use std::time::{SystemTime};//, UNIX_EPOCH};

const DATA_SIZE: usize = 1024 * 1024;
const LOOPS_COUNT: u32 = 16 * 1024;
//const SHA256_DIGEST_LENGTH: usize = 32;

fn main() {
    let data = [0u8; DATA_SIZE];
    //let mut hash = [0u8; SHA256_DIGEST_LENGTH];
    let mut _hash = Sha256::digest(data);

    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        _hash = Sha256::digest(data);
    }
    println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}

