#[macro_use(debug, error, info, log, log_enabled, trace, warn)]
extern crate log;
extern crate rustc_serialize;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

pub use std::env;
// pub use std::time::{Duration, SystemTime};
pub use std::collections::{BTreeMap, HashMap};
pub use rustc_serialize::json;

#[path="./models/mod.rs"]
pub mod models;
#[path="./utils/mod.rs"]
pub mod utils;


fn main() {
    env::set_var("mongo_db_uri", "mongodb://localhost:27017");
    
}
