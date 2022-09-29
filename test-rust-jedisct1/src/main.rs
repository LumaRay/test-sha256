// https://github.com/jedisct1/rust-hmac-sha256
// 68292

use std::time::{SystemTime};//, UNIX_EPOCH};

mod lib;

const DATA_SIZE: usize = 1024 * 1024;
const LOOPS_COUNT: u32 = 16 * 1024;
//const SHA256_DIGEST_LENGTH: u32 = 32;

fn main() {
    let mut data = [0u8; DATA_SIZE];
    //let mut hash = [0u8; SHA256_DIGEST_LENGTH];

    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        //let mut h = Hash::new();
        //h.update(data);
        //let hash = h.finalize();
        let _hash = lib::Hash::hash(&data);
    }
    println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}

