use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::Write;
use std::str;
use std::path::Path;
use std::path::PathBuf;

// extern crate walkdir;
use walkdir::WalkDir;

use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use sha2::{Sha256, Digest};

mod util;

#[derive(Debug)]
/// A struct that represents a key-value store.
pub struct KVStore {
    /// The number of key-value mappings currently stored.
    size: usize,
    /// The location of the file system where key-value mappings are stored.
    path: String,
}

/// A trait that defines the operations that need to be supported.
pub trait Operations {
    /// A function that initializes a KVStore instance.
    ///
    /// If there is no directory at the provided path, this should create it. If there is an error
    /// while creating a directory, this should return an [std::io::Error].
    ///
    /// If there are **no** key-value mappings stored already under the directory, this
    /// should simply create a new KVStore instance that can store and retrieve key-value mappings
    /// using the directory. It should also correctly initialize the size to 0.
    ///
    /// If there **are** existing key-value mappings stored already under the directory, this
    /// should initialize a KVStore instance that is able to store and retrieve existing key-value
    /// mappings as well as new key-value mappings. It should also correctly initialize the size to
    /// the number of existing key-value mappings.
    fn new(path: &str) -> std::io::Result<Self>
    where
        Self: Sized;

    /// A function that returns the number of key-value mappings currently stored.
    fn size(self: &Self) -> usize;

    /// A function that inserts a new key-value mapping.
    ///
    /// If there is **no** key-value mapping stored already with the same key, it should return
    /// `Ok(())` if storing is successfully done.
    ///
    /// If there **is** a key-value mapping stored already with the same key, it should return an
    /// [std::io::Error].
    ///
    /// Make sure you read and understand the assignment document regarding how to store key-value
    /// mappings using files as well as how to structure sub-directories.
    ///
    /// Make sure you understand what the trait bounds mean for K and V.
    ///
    /// Refer to [https://docs.serde.rs/serde/](https://docs.serde.rs/serde/)
    /// and [https://serde.rs](https://serde.rs) for serde.
    fn insert<K, V>(self: &mut Self, key: K, value: V) -> std::io::Result<()>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::Serialize + Default + Debug;

    /// A function that returns a previously-inserted value.
    ///
    /// If there **is** a key-value mapping stored already with the same key, it should return
    /// the value.
    ///
    /// If there is **no** key-value mapping stored already with the same key, it should return
    /// an [std::io::Error].
    ///
    /// Make sure you understand what the trait bounds mean for K and V.
    ///
    /// Refer to [https://docs.serde.rs/serde/](https://docs.serde.rs/serde/)
    /// and [https://serde.rs](https://serde.rs) for serde.
    fn lookup<K, V>(self: &Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug;

    /// A function that removes a previously-inserted key-value mapping.
    ///
    /// If there **is** a key-value mapping stored already with the same key, it should return
    /// the value and delete the key-value mapping from the file system.
    ///
    /// If there is **no** key-value mapping stored already with the same key, it should
    /// return an [std::io::Error].
    ///
    /// If a sub-directory does not contain any key-value files, this should delete the
    /// sub-directory as well.
    ///
    /// Make sure you understand what the trait bounds mean for K and V.
    ///
    /// Refer to [https://docs.serde.rs/serde/](https://docs.serde.rs/serde/)
    /// and [https://serde.rs](https://serde.rs) for serde.
    fn remove<K, V>(self: &mut Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug;
}

impl Operations for KVStore {
    fn new(path: &str) -> std::io::Result<Self>
    where
        Self: Sized 
    {
        let _path = Path::new(path);
        // create directories if provided path does not exist
        if !_path.exists() { fs::create_dir_all(path)? };
        // count the number of keys
        let count: usize = util::count_keys(_path)?;

        Ok(KVStore {
            size: count,
            path: String::from(path)
        })
    }

    fn size(self: &Self) -> usize {
        self.size
    }

    fn insert<K, V>(self: &mut Self, key: K, value: V) -> std::io::Result<()>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::Serialize + Default + Debug 
    {
        println!("inserted {:?}, {:?} to {:?}", key, value, self.path);
        
        // 1) Serialize key and value using serde (use serde_json)
        let key_json = serde_json::to_string(&key)?;     // converts to io::Error
        let value_json = serde_json::to_string(&value)?; // converts to io::Error

        // 2) Generate SHA256 Digest string (hash)
        let mut hasher = Sha256::new();
        hasher.update(&key_json);
        let key_hash : String = format!("{:x}", hasher.finalize());        
        
        //3) check if keys exist (find .key file with same hash, check key)
        let key_file = key_hash.clone() + ".key";
        let key_folder = &key_hash.clone()[..10];
        let key_path = format!("/{}/{}", key_folder, key_file);
        let key_file_path = format!("{}{}", self.path, key_path);
        let file_exists =  Path::new(&key_file_path).exists();

        // 4) IF it exists, check if key value in file is same as key        
        if (file_exists) {
          let file_key = fs::read_to_string(&key_file_path).expect("Could not read existing key file!");
          // println!("File Key : {:?}", file_key);
          // println!("Input Key: {:?}", key_json);
          assert_eq!(file_key.eq(&key_json), false, "Checking that existing key <{:?}> is not equal to input key <{:?}>", file_key, key_json);
          fs::write(key_file_path, &key_json).expect("Could not write new key to file!");
        } else {
        // 3) Generate 2 files (<hash>.key and <hash>.value)
          let value_file = key_hash.clone() + ".value";
          let value_path = format!("/{}/{}", key_folder, value_file);
          let value_file_path = format!("{}{}", self.path, value_path);          
          let mut path_key = Path::new(&key_file_path);
          let prefix_key = path_key.parent().unwrap();
          fs::create_dir(prefix_key)?;
          fs::write(key_file_path, key_json)?;
          fs::write(value_file_path, value_json)?;
        }
        return Ok(()) // storing successful
    }

    fn lookup<K, V>(self: &Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug 
    {
      let key_json = serde_json::to_string(&key)?;

      // Generate digest string (to search for hash)
      let mut hasher = Sha256::new();
      hasher.update(&key_json);
      let key_hash : String = format!("{:x}", hasher.finalize());

      // Search for folder / file
      let key_file = key_hash.clone() + ".key";
      let key_folder = &key_hash.clone()[..10];
      let key_path = format!("/{}/{}", key_folder, key_file);
      let key_file_path = format!("{}{}", self.path, key_path);
      let file_exists =  Path::new(&key_file_path).exists();

      // get value
      assert_eq!(file_exists, true, "The key : <{:?}> does not exist in KVstore!", key_json);

      let value_file = key_hash.clone() + ".value";
      let value_path = format!("/{}/{}", key_folder, value_file);
      let value_file_path = format!("{}{}", self.path, value_path);
      let value_content = fs::read_to_string(value_file_path.clone()).expect("Could not read existing key file!");
      // let return_value = V::from(serde_json::from_str(&value_content));
      // let deserealized = serde_json::from_str(&value_content);
      let return_value : V = serde_json::from_str(&value_content).unwrap();

      Ok(return_value)
    }

    fn remove<K, V>(self: &mut Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug 
    {
        
        let key_json = serde_json::to_string(&key)?;

        // Generate digest string (to search for hash)
        let mut hasher = Sha256::new();
        hasher.update(&key_json);
        let key_hash : String = format!("{:x}", hasher.finalize());

        // Search for folder / file
        let key_file = key_hash.clone() + ".key";
        let key_folder = &key_hash.clone()[..10];
        let key_path = format!("/{}/{}", key_folder, key_file);
        let key_file_path = format!("{}{}", self.path, key_path);
        let file_exists =  Path::new(&key_file_path).exists();

        // get value
        assert_eq!(file_exists, true, "The key : <{:?}> does not exist in KVstore!", key_json);

        let value_file = key_hash.clone() + ".value";
        let value_path = format!("/{}/{}", key_folder, value_file);
        let value_file_path = format!("{}{}", self.path, value_path);
        let value_content = fs::read_to_string(value_file_path.clone()).expect("Could not read existing key file!");
        // let return_value = V::from(serde_json::from_str(&value_content));
        // let deserealized = serde_json::from_str(&value_content);
        let return_value : V = serde_json::from_str(&value_content).unwrap();
        // Remove key and value file
        fs::remove_file(key_file_path)?;
        fs::remove_file(value_file_path)?;

        // Check if folder is empty
        let folder_path =format!("{}/{}", self.path, key_folder);
        let folder_empty = fs::read_dir(folder_path.clone())?.next().is_none();

        if (folder_empty) {
          fs::remove_dir(folder_path)?;
        }


        Ok(return_value)
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
