
extern crate bson;

pub mod sha1;
pub mod uuid;

use std::fmt::Write;
// pub use self::sha1::gen;
// pub use self::uuid::gen;

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::new();
    for &byte in bytes.iter() {
        write!(&mut hex, "{:X}", byte).unwrap();
    }
    hex
}

pub use self::bson::oid::ObjectId;

// Gen:
//      let oid: Result<ObjectId> = ObjectId::new()
// With String:
//      let oid: Result<ObjectId, _> = ObjectId::with_string("asdsdasdasd")

