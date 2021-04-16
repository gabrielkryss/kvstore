#![no_main]
use libfuzzer_sys::fuzz_target;

use kvstore::{KVStore, Operations};
use serde::{Serialize, Deserialize};

fn test_1(data: i32) {
    let mut kvs = KVStore::new(".").unwrap();
    kvs.insert(data, data).unwrap();
    kvs.lookup::<i32, i32>(data).unwrap();
    kvs.remove::<i32, i32>(data).unwrap();
}

fn test_2(data: i32) {
    let mut kvs = KVStore::new(".").unwrap();
    kvs.insert(String::from(data.to_string()), vec![data]).unwrap();
    kvs.lookup::<String, Vec<i32> >(String::from(data.to_string())).unwrap();
    kvs.remove::<String, Vec<i32> >(String::from(data.to_string())).unwrap();
}

fn test_3(data: i32) {

    #[derive(Serialize, Deserialize, Debug, Default)]
    struct Entity {
        x: i32
    }

    let mut kvs = KVStore::new(".").unwrap();
    kvs.insert(String::from(data.to_string()), Entity{x: data}).unwrap();
    kvs.lookup::<String, Entity >(String::from(data.to_string())).unwrap();
    kvs.remove::<String, Entity >(String::from(data.to_string())).unwrap();
}

fuzz_target!(|data: i32| {
    test_1(data);
    test_2(data);
    test_3(data);
});
