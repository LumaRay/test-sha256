pub trait SHA256HashPrototype {
    fn update(&mut self, input: &[u8]) -> bool;
    fn digest(&mut self) -> [u32; 8];
}
use byteorder::{BigEndian, ReadBytesExt};
use std::convert::TryInto;
const ROUND_CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
pub struct SHA256Hash {
    h0: u32,
    h1: u32,
    h2: u32,
    h3: u32,
    h4: u32,
    h5: u32,
    h6: u32,
    h7: u32,
    finalized: bool,
    total_bits: usize,
    unprocessed_bytes: Vec<u8>,
    input_block_size: usize,
}

impl SHA256HashPrototype for SHA256Hash {
    fn digest(&mut self) -> [u32; 8] {
        self.finalize();
        [
            self.h0, self.h1, self.h2, self.h3, self.h4, self.h5, self.h6, self.h7,
        ]
    }

    fn update(&mut self, input: &[u8]) -> bool {
        if self.finalized || input.len() == 0 {
            return false;
        }
        //self.total_bytes += input.len();

        self.consume(input);

        true
    }
}
impl SHA256Hash {
    pub fn new() -> SHA256Hash {
        SHA256Hash {
            h0: 0x6a09e667,
            h1: 0xbb67ae85,
            h2: 0x3c6ef372,
            h3: 0xa54ff53a,
            h4: 0x510e527f,
            h5: 0x9b05688c,
            h6: 0x1f83d9ab,
            h7: 0x5be0cd19,
            finalized: false,
            total_bits: 0,
            unprocessed_bytes: Vec::with_capacity(64),
            input_block_size: 64,
        }
    }
    pub fn block_size(&self) -> usize {
        self.input_block_size
    }

    /// update with a block of 512bit
    fn update_block(&mut self, block: &[u8]) {
        let mut w: [u32; 64] = [0; 64];

        for j in 0..16 {
            // let mut bytes: [u8; 4] = Default::default();
            // bytes.copy_from_slice(&block[j*4..(j+1)*4]);
            // w[j] = transform_array_of_u8_big_endian_to_u32(&bytes);
            w[j] = transform_array_of_u8_big_endian_to_u32(
                block[j * 4..(j + 1) * 4].try_into().expect(""),
            );
            // println!("w[{:2}] {:08X?}", j, w[j]);
        }

        // Extend the first 16 words into the remaining 48 words w[16..63] of the message schedule array:
        for j in 16..64 {
            let s0 = w[j - 15].rotate_right(7) ^ w[j - 15].rotate_right(18) ^ (w[j - 15] >> 3);
            let s1 = w[j - 2].rotate_right(17) ^ w[j - 2].rotate_right(19) ^ (w[j - 2] >> 10);
            w[j] = w[j - 16]
                .wrapping_add(s0)
                .wrapping_add(w[j - 7])
                .wrapping_add(s1);
        }

        let mut a = self.h0;
        let mut b = self.h1;
        let mut c = self.h2;
        let mut d = self.h3;
        let mut e = self.h4;
        let mut f = self.h5;
        let mut g = self.h6;
        let mut h = self.h7;

        // Compression function main loop:
        for j in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(ROUND_CONSTANTS[j])
                .wrapping_add(w[j]);

            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.h0 = self.h0.wrapping_add(a);
        self.h1 = self.h1.wrapping_add(b);
        self.h2 = self.h2.wrapping_add(c);
        self.h3 = self.h3.wrapping_add(d);
        self.h4 = self.h4.wrapping_add(e);
        self.h5 = self.h5.wrapping_add(f);
        self.h6 = self.h6.wrapping_add(g);
        self.h7 = self.h7.wrapping_add(h);
    }

    /// consume blocks of unprocessed bytes
    fn consume(&mut self, mut bytes: &[u8]) {
        let input_len_bytes = bytes.len();
        let unprocessed_len = self.unprocessed_bytes.len();
        self.total_bits += input_len_bytes * 8;
        // Do we have bytes in the unprocessed buffer?
        if unprocessed_len > 0 {
            if (unprocessed_len + input_len_bytes) < 64 {
                // Nothing to do, we just append
                // Copy up to 63 bytes to our Vec
                self.unprocessed_bytes.extend_from_slice(bytes);
                return;
            }
            let (additional, new_bytes) = bytes.split_at(64 - unprocessed_len);
            // Reassign
            bytes = new_bytes;
            // Copy up to 64 bytes from what we just took
            self.unprocessed_bytes.extend_from_slice(additional);
            // We can afford a 64-byte clone
            self.update_block(self.unprocessed_bytes.clone().as_slice());
            self.unprocessed_bytes.clear();
            // Call ourselves
            //return self.inner_consume(new_bytes);
        }
        let iter = bytes.chunks_exact(64);
        let remainder_i = iter.clone();
        for block in iter {
            self.update_block(&block)
        }
        let bytes = remainder_i.remainder();
        self.unprocessed_bytes.extend_from_slice(bytes); // max 64bytes allocated
    }

    /// query the hex digest, formatted as hexadecimal string
    pub fn hex_digest(&mut self) -> String {
        let digest = self.digest();
        format!(
            "{:08X?}{:08X?}{:08X?}{:08X?}{:08X?}{:08X?}{:08X?}{:08X?}",
            digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7]
        )
    }

    fn finalize(&mut self) {
        let mut new_bytes: Vec<u8> = vec![0x80];
        let n_padding_bits = 512 - (self.total_bits + 8 + 64) % 512;
        let n_padding_bytes = n_padding_bits / 8;
        new_bytes.extend(vec![0; n_padding_bytes]);
        let length: u64 = self.total_bits.try_into().unwrap();
        new_bytes.extend(&transform_u64_to_array_of_u8_big_endian(length));

        self.consume(&new_bytes);

        self.finalized = true;
    }
}

///////////////////////////////////////////////////////////////////////////////

fn transform_u64_to_array_of_u8_big_endian(x: u64) -> [u8; 8] {
    let b1 = ((x >> 56) & 0xff) as u8;
    let b2 = ((x >> 48) & 0xff) as u8;
    let b3 = ((x >> 40) & 0xff) as u8;
    let b4 = ((x >> 32) & 0xff) as u8;
    let b5 = ((x >> 24) & 0xff) as u8;
    let b6 = ((x >> 16) & 0xff) as u8;
    let b7 = ((x >> 8) & 0xff) as u8;
    let b8 = (x & 0xff) as u8;
    [b1, b2, b3, b4, b5, b6, b7, b8]
}

fn transform_array_of_u8_big_endian_to_u32(mut arr_of_u8: &[u8]) -> u32 {
    arr_of_u8.read_u32::<BigEndian>().unwrap()
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::SHA256Hash;
    use crate::SHA256HashPrototype;

    /// Run empty test input from FIPS 180-2
    #[test]
    fn sha256_nist_empty() {
        let mut hasher = SHA256Hash::new();
        hasher.update(&[]);
        let digest = hasher.digest();
        assert_eq!(
            digest,
            [
                0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924, 0x27ae41e4, 0x649b934c, 0xa495991b,
                0x7852b855
            ]
        );
    }

    /// Run abc test from FIPS 180-2
    #[test]
    fn sha256_nist_abc() {
        let mut hasher = SHA256Hash::new();
        hasher.update(b"abc");
        let digest = hasher.digest();
        assert_eq!(
            digest,
            [
                0xba7816bf, 0x8f01cfea, 0x414140de, 0x5dae2223, 0xb00361a3, 0x96177a9c, 0xb410ff61,
                0xf20015ad
            ]
        )
    }

    /// Run two-block test from FIPS 180-2
    #[test]
    fn sha256_nist_two_blocks() {
        let mut hasher = SHA256Hash::new();
        hasher.update(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
        let digest = hasher.digest();
        assert_eq!(
            digest,
            [
                0x248d6a61, 0xd20638b8, 0xe5c02693, 0x0c3e6039, 0xa33ce459, 0x64ff2167, 0xf6ecedd4,
                0x19db06c1
            ]
        );
    }

    /// Run large input test (1,000,000 x a) from FIPS 180-2
    #[test]
    fn sha256_nist_large_input() {
        let input_str = std::iter::repeat("a").take(1_000_000).collect::<String>();
        let input = input_str.as_bytes();
        let mut hasher = SHA256Hash::new();
        hasher.update(&input);
        let digest = hasher.digest();
        assert_eq!(
            digest,
            [
                0xcdc76e5c, 0x9914fb92, 0x81a1c7e2, 0x84d73e67, 0xf1809a48, 0xa497200e, 0x046d39cc,
                0xc7112cd0
            ]
        );
    }
}

