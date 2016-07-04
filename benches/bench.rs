#![feature(test)]
extern crate test;
#[macro_use(bson, doc)]
extern crate bson;
extern crate uuid;
extern crate crypto;

use std::fmt::Write;

use test::Bencher;
use bson::oid::ObjectId;
use uuid::{Uuid, UuidVersion};

use crypto::sha1::Sha1;
// use crypto::sha2::Sha2;
use crypto::sha3::Sha3;
use crypto::digest::Digest;

#[bench]
fn bench_mongo_object_id(b: &mut Bencher) {
    b.iter(| |{
        ObjectId::new();
    });
}

#[bench]
fn bench_uuid_v1(b: &mut Bencher){
    b.iter(| |{
        // return None
        let uuid_v1 = Uuid::new(UuidVersion::Mac);
    });
}
#[bench]
fn bench_uuid_v2(b: &mut Bencher){
    b.iter(| |{
        // return None
        let uuid_v2 = Uuid::new(UuidVersion::Dce);
    });
}
#[bench]
fn bench_uuid_v3(b: &mut Bencher){
    b.iter(| |{
        // return None
        let uuid_v3 = Uuid::new(UuidVersion::Md5);
    });
}
#[bench]
fn bench_uuid_v4(b: &mut Bencher){
    b.iter(| |{
        // let uuid_v4 = Uuid::new_v4();
        let uuid_v4 = Uuid::new(UuidVersion::Random);
    });
}
#[bench]
fn bench_uuid_v5(b: &mut Bencher){
    b.iter(| |{
        // return None
        let uuid_v5 = Uuid::new(UuidVersion::Sha1);
    });
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::new();
    for &byte in bytes.iter() {
        write!(&mut hex, "{:X}", byte).unwrap();
    }
    hex
}

#[bench]
fn bench_sha1(b: &mut Bencher){
    b.iter(| |{
        let msg   = "The quick brown fox jumps over the lazy dog.测试。";
        let mut m = Sha1::new();
        m.input(msg.as_bytes());
        // let output_str = "8e5bacdac0ef009d28f8cc2ce326ca1d30c7a2c1";
        let mut output = [0; 20];
        m.result(&mut output);
        let hex = bytes_to_hex(&output);
    });
}

