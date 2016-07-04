extern crate crypto;

// use std::fmt::Write;

pub use self::crypto::sha1::Sha1;
// use crypto::sha2::Sha2;
pub use self::crypto::sha3::Sha3;
pub use self::crypto::digest::Digest;

use super::bytes_to_hex;

pub fn gen (msg: &str) -> String {
    let mut m = Sha1::new();
    m.input(msg.as_bytes());
    let mut output = [0; 20]; // 8e5bacdac0ef009d28f8cc2ce326ca1d30c7a2c1
    m.result(&mut output);
    bytes_to_hex(&output)
}