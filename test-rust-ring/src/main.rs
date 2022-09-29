// https://github.com/briansmith/ring/blob/main/src/digest/sha2.rs
// https://docs.rs/ring/latest/ring/
// 42541

use ring::digest;
use std::time::{SystemTime};//, UNIX_EPOCH};

const DATA_SIZE: usize = 1024 * 1024;
const LOOPS_COUNT: u32 = 16 * 1024;
//const SHA256_DIGEST_LENGTH: usize = 32;

fn main() {
    let data = [0u8; DATA_SIZE];
    //let mut hash = [0u8; SHA256_DIGEST_LENGTH];
    let mut _hash = digest::digest(&digest::SHA256, &data);

    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        _hash = digest::digest(&digest::SHA256, &data);
        //println!("{:?}", digest::digest(&digest::SHA256, b"hello, world"));
        //let mut ctx = digest::Context::new(&digest::SHA256);
        //ctx.update(&data);
        //let i_u_f = ctx.finish();
        //println!("{:?}", i_u_f.as_ref());
    }
    println!("Test {:?}", SystemTime::now().duration_since(start).unwrap());
}

