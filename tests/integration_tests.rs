use kvstore::{KVStore, Operations};
use serial_test::serial;

#[test]
#[serial]
fn insert_val_int_test() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    assert_eq!(kvs.size(), 0);
    
    kvs.insert(String::from("testkey_123"), 45 as i32).unwrap();
    let lookup_val = kvs.lookup::<String, i32>(String::from("testkey_123")).unwrap();    
    assert_eq!(lookup_val, 45 as i32);
    // Cleanup
    let cleanup_val = kvs.remove::<String, i32>(String::from("testkey_123")).unwrap();
    assert_eq!(cleanup_val, 45 as i32);
    assert_eq!(kvs.size(), 0);
}

#[test]
#[serial]
fn insert_val_string_test() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    assert_eq!(kvs.size(), 0);
    
    kvs.insert(String::from("testkey_123"), String::from("value_string")).unwrap();
    let lookup_val = kvs.lookup::<String, String>(String::from("testkey_123")).unwrap();    
    assert_eq!(lookup_val, String::from("value_string"));
    // Cleanup
    let cleanup_val = kvs.remove::<String, String>(String::from("testkey_123")).unwrap();
    assert_eq!(cleanup_val, String::from("value_string"));
    assert_eq!(kvs.size(), 0);
}

#[test]
#[serial]
fn insert_val_bool_test() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    assert_eq!(kvs.size(), 0);
    
    kvs.insert(String::from("testkey_123"), true).unwrap();
    let lookup_val = kvs.lookup::<String, bool>(String::from("testkey_123")).unwrap();    
    assert_eq!(lookup_val, true);
    // Cleanup
    let cleanup_val = kvs.remove::<String, bool>(String::from("testkey_123")).unwrap();
    assert_eq!(cleanup_val,true);
    assert_eq!(kvs.size(), 0);
}


#[test]
#[serial]
fn remove_key_test() { 
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    assert_eq!(kvs.size(), 0);

    kvs.insert(String::from("testkey_123"), 45 as i32).unwrap();
    let rem_val = kvs.remove::<String, i32>(String::from("testkey_123")).unwrap();
    assert_eq!(rem_val, 45 as i32);
    // Cleanup
    assert_eq!(kvs.size(), 0);    
}

#[test]
#[serial]
fn insert_existing_key_err() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    assert_eq!(kvs.size(), 0);

    kvs.insert(String::from("testkey_123"), 45 as i32).unwrap();

    let duplicate_result = kvs.insert(String::from("testkey_123"), 45 as i32).unwrap_err().kind();
    let expected_err = std::io::ErrorKind::Other;
    assert_eq!(expected_err, duplicate_result);

    // Cleanup
    let cleanup_val = kvs.remove::<String, i32>(String::from("testkey_123")).unwrap();
    assert_eq!(cleanup_val, 45 as i32);
    assert_eq!(kvs.size(), 0);
}

#[test]
#[serial]
fn remove_nonexisting_key_err() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    assert_eq!(kvs.size(), 0);

    let remove_result = kvs.remove::<String, i32>(String::from("testkey_123")).unwrap_err().kind();
    let expected_err = std::io::ErrorKind::Other;
    assert_eq!(expected_err, remove_result);

    // Cleanup
    assert_eq!(kvs.size(), 0);
}

#[test]
#[serial]
fn lookup_nonexisting_key_err() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    assert_eq!(kvs.size(), 0);

    let lookup_result = kvs.lookup::<String, i32>(String::from("testkey_123")).unwrap_err().kind();
    let expected_err = std::io::ErrorKind::Other;
    assert_eq!(expected_err, lookup_result);

    // Cleanup
    assert_eq!(kvs.size(), 0);
}

#[test]
#[serial]
fn check_size_after_remove() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    assert_eq!(kvs.size(), 0);

    kvs.insert(String::from("testkey_123"), 123 as i32).unwrap();
    kvs.insert(String::from("testkey_234"), 234 as i32).unwrap();
    kvs.insert(String::from("testkey_567"), 567 as i32).unwrap();
    kvs.remove::<String, i32>(String::from("testkey_123")).unwrap();
    assert_eq!(kvs.size(), 2);

    // Cleanup
    kvs.remove::<String, i32>(String::from("testkey_234")).unwrap();
    kvs.remove::<String, i32>(String::from("testkey_567")).unwrap();
    assert_eq!(kvs.size(), 0);
}

#[test]
#[serial]
fn check_size_after_insert() {
    let mut kvs = KVStore::new("./database/instance_1").unwrap();
    assert_eq!(kvs.size(), 0);

    kvs.insert(String::from("testkey_123"), 123 as i32).unwrap();
    kvs.insert(String::from("testkey_234"), 234 as i32).unwrap();
    kvs.insert(String::from("testkey_567"), 567 as i32).unwrap();    
    assert_eq!(kvs.size(), 3);

    // Cleanup
    kvs.remove::<String, i32>(String::from("testkey_123")).unwrap();
    kvs.remove::<String, i32>(String::from("testkey_234")).unwrap();
    kvs.remove::<String, i32>(String::from("testkey_567")).unwrap();
    assert_eq!(kvs.size(), 0);  
}

