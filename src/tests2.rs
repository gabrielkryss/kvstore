use kvstore::{KVStore, Operations};
// use std::fs;
use serde::{Serialize, Deserialize};

//test case 1. Create a new store. Test that it has 0 entries.
#[test]
fn test1() {
    let kvs = KVStore::new("test_stores/test1").unwrap();
    assert_eq!(0,kvs.size());
}

//test case 2. Set kvs to an already existing, but empty store and add a pair to it.
#[test]
fn test2() {
    let mut kvs = KVStore::new("test_stores/test2").unwrap();
    match kvs.insert(String::from("Key"), String::from("Value")){
        Ok(_) => (),
        Err(e) => println!("{:?}",e ),
    }
    assert_eq!(1,kvs.size());
}

//test case 3. Set kvs to an already existing non-empty store and check its size.
#[test]
fn test3() {
    let mut kvs = KVStore::new("test_stores/test3").unwrap();
    match kvs.insert(String::from("Key"), String::from("Value")){
        Ok(_) => (),
        Err(e) => println!("{:?}",e ),
    }
    let kvs2 = KVStore::new("test_stores/test3").unwrap();
    assert_eq!(1,kvs2.size());
}



//test case 4. Testing for lookup.
#[test]
fn test4() {
    let mut kvs = KVStore::new("test_stores/test4").unwrap();
    let key = String::from("Key");
    let value = String::from("Value");
    match kvs.insert(key,value){
        Ok(_) => (),
        Err(e) => println!("{:?}",e ),
    }
    let looked_up = kvs.lookup::<String,String>(String::from("Key")).unwrap();
    assert_eq!(looked_up,String::from("Value"));
}

//test case 5. More testing for lookup.
#[test]
fn test5() {
    let mut kvs = KVStore::new("test_stores/test5").unwrap();
    let key = 2 as u32;
    let value = 5 as usize;
    match kvs.insert(key,value){
        Ok(_) => (),
        Err(e) => println!("{:?}",e ),
    }
    let looked_up = kvs.lookup::<u32,usize>(key).unwrap();
    assert_eq!(looked_up,value);
}
//test case 6. Retrieving a value from an already existing store
#[test]
fn test6() {
    let mut kvs = KVStore::new("test_stores/test6").unwrap();
    let key = 2 as i32;
    let value = 5 as u32;
    match kvs.insert(key,value){
        Ok(_) => (),
        Err(e) => println!("{:?}",e ),
    }
    let kvs2 = KVStore::new("test_stores/test6").unwrap();
    let looked_up = kvs2.lookup::<i32,u32>(key).unwrap();
    assert_eq!(looked_up,value);
}
//test case 7. Inserting / Retrieving Structs
#[test]
fn test7() {
    #[derive(Serialize,Deserialize,Default,Debug,PartialEq)]
    struct Car{
        color: String,
        wheels: usize,
    }
    let mut kvs = KVStore::new("test_stores/test7").unwrap();
    let key = 1 as usize;
    let value = Car{color:"Red".to_string(), wheels:3};
    match kvs.insert(key,value){
        Ok(_) => (),
        Err(e) => println!("{:?}",e ),
    }
    let looked_up = kvs.lookup::<usize,Car>(key).unwrap();
    assert_eq!(looked_up,Car{color:"Red".to_string(), wheels:3});
}

//test case 8. Removing from the store
#[test]
fn test8() {
    let mut kvs = KVStore::new("test_stores/test8").unwrap();
    let key = 1 as f64;
    let value = String::from("Hi");
    match kvs.insert(key,value){
        Ok(_) => (),
        Err(e) => println!("{:?}",e ),
    }
    let looked_up = kvs.remove::<f64,String>(key).unwrap();
    assert_eq!(looked_up,"Hi");

    assert_eq!(kvs.size(),0);
}

