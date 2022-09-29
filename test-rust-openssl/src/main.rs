// https://docs.rs/openssl/latest/openssl/sha/index.html
// 37666

use openssl::sha::sha256;
use std::time::{SystemTime};//, UNIX_EPOCH};

const DATA_SIZE: usize = 1024 * 1024;
const LOOPS_COUNT: u32 = 16 * 1024;
//const SHA256_DIGEST_LENGTH: usize = 32;

fn main() {
    let data = [0u8; DATA_SIZE];
    //let mut hash = [0u8; SHA256_DIGEST_LENGTH];

    //let mut hasher = Sha256::new();
    //hasher.update(data);
    //let mut _hash = hasher.finalize_reset();
    //hasher.reset();
    let mut _hash = sha256(&data);

    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        //hasher.update(data);
        //_hash = hasher.finalize();
        //_hash = hasher.finalize_reset();
        _hash = sha256(&data);
        //hasher.reset();
        //hasher.finalize_into_reset(&hash);
    }
    println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}

