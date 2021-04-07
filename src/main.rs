//! This is just an example.
use kvstore::{KVStore, Operations};

fn main() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    kvs.insert(String::from("testkey_123"), 1 as i32).unwrap();
    kvs.insert(String::from("testkey_234"), 1 as i32).unwrap();
    // kvs.lookup::<String, i32>(String::from("key")).unwrap();
    // kvs.remove::<String, i32>(String::from("key")).unwrap();
}
