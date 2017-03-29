#[test]
extern crate crypto;

use crypto::md5::Md5;
use std::mem::transmute;

// number of the point-of-presence for each MD5 digest is 4.
// A MD5 digest is 128 bit, and the continuum is built on the entire 32-bit
// integer space, hence each MD5 digest is evenly split into 4 32-bit integers.
const NPOP_PER_MD5: u32 = 4;

// Here we assume all hash values are 64-bit floating point numbers (f64)
// This is the finagle implementation, the small value added is missing in
// other implementations e.g. libketama (in C), so they may not be compatible
fn ndigest_finagle(weight: u32, sum: u32, npop: u32) -> u32 {
    let w_d = weight as f64;
    let s_d = sum as f64;
    let pct = w_d / s_d;

    (pct * npop / NPOP_PER_MD5 + 0.000000000001) as u32
}

fn get_pop(id: String) -> [u32; ..4] {
    let mut hasher = Md5::new();
    let mut digest : [u8; ..16] = [0; 16];
    hasher.input(id.as_bytes());
    hasher.result(&mut digest);

    let pop : [u32; ..4] = unsafe { transmute(digest) };
    pop
}
