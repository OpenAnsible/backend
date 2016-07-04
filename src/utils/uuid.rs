
extern crate uuid;

use self::uuid::{Uuid}; // UuidVersion
use super::bytes_to_hex;

pub fn gen() -> String {
    let uuid_v4 = Uuid::new_v4();    // Uuid::new(UuidVersion::Random)
    bytes_to_hex(uuid_v4.as_bytes())
}