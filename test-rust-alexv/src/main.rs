// https://codereview.stackexchange.com/questions/230384/educational-implementation-of-sha256-in-rust
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=fb9ec52bdd7ace8ceeb2e8d79c5ef051
// 88485

use sha256::SHA256Hash;
use sha256::SHA256HashPrototype;
use std::time::{SystemTime};//, UNIX_EPOCH};

const DATA_SIZE: usize = 1024 * 1024;
const LOOPS_COUNT: u32 = 16 * 1024;
//const SHA256_DIGEST_LENGTH: usize = 32;

fn main() {
    let data = [0u8; DATA_SIZE];
    //let mut hash = [0u8; SHA256_DIGEST_LENGTH];

    let mut hasher = SHA256Hash::new();
    let _chunk_size: usize = hasher.block_size() * 1024;
    hasher.update(&data);
    //hasher.reset();
    let mut _hash = hasher.hex_digest();

    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        hasher = SHA256Hash::new();
        hasher.update(&data);
        //_hash = hasher.finalize();
        //_hash = hasher.finalize_reset();
        _hash = hasher.hex_digest();
        //hasher.reset();
        //hasher.finalize_into_reset(&hash);
    }
    println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}

