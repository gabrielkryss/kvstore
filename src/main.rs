//! This is just an example.
use kvstore::{KVStore, Operations};

fn main() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    kvs.insert(String::from("testkey_123"), 45 as i32).unwrap();
    kvs.insert(String::from("testkey_124"), 45 as i32).unwrap();
    kvs.insert(String::from("testkey_234"), 298 as i32).unwrap();
    // kvs.lookup::<String, i32>(String::from("key")).unwrap();
    println!("Retrieving value : {:x}", kvs.remove::<String, i32>(String::from("testkey_123")).unwrap());
}
