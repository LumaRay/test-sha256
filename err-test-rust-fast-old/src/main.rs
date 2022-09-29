// old https://github.com/ActualIlluminati/sha256fast
// old https://crates.io/crates/sha256fast
// 

//use sha256fast::{Digest, Sha256};
use std::time::{SystemTime};//, UNIX_EPOCH};

mod lib;

const DATA_SIZE: usize = 1024 * 1024;
const LOOPS_COUNT: u32 = 16 * 1024;
const SHA256_DIGEST_LENGTH: usize = 32;

fn main() {
    let data = [0u8; DATA_SIZE];
    let mut hash = [0u8; SHA256_DIGEST_LENGTH];

    let mut digest = digest_init();

    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        for i in 0..data.len() / 64 / 8 {
            transform_8way(&mut digest, data.as_ptr().add(i * 64 * 8));
        }
        digest_finish(&mut hash, digest);
    }
    println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}

