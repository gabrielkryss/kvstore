//! This is just an example.
use kvstore::{KVStore, Operations};

fn main() {
    let kvs = KVStore::new(".").unwrap();
    kvs.insert("key", "value").unwrap();
    kvs.lookup::<&str, &str>("key").unwrap();
    kvs.remove::<&str, &str>("key").unwrap();
}
