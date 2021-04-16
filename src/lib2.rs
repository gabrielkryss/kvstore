use std::fmt::Debug;
use std::fs;
use walkdir::WalkDir;

use std::ffi::OsStr;
use std::fs::File;
// use serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind};
use sha256::digest;
use std::fs::read_dir;
use std::path::Path;
#[derive(Debug)]
/// A struct that represents a key-value store.
pub struct KVStore2 {
    /// The number of key-value mappings currently stored.
    size: usize,
    /// The location of the file system where key-value mappings are stored.
    path: String,
}
pub trait TypeToStr{
    fn get_type(&self) -> String;
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


//given the path to an existing directory, find the number of .key and .value files and divide by 2
pub fn calc_num_of_pairs(path: &str) -> usize{
    let mut counter = 0;
    for entry in WalkDir::new(path){
        let entry = entry.unwrap();
        let path = entry.path().extension().and_then(OsStr::to_str);
        match path{
            Some(p) => {
                // println!("found {}",p);
                if p == "key" || p == "value" {
                    // println!("adding 1 to counter");
                    counter = counter + 1;
                }
            },
            None => (),
            // None => println!("Error"),
        }
    }
    counter / 2
}




impl Operations for KVStore2{
    fn new(path: &str) -> std::io::Result<Self>
    where
        Self: Sized{

        if Path::new(path).is_dir(){
            return Ok(KVStore{size:calc_num_of_pairs(path), path: String::from(path)})
        }
        else{
            return Ok(KVStore{size:0, path: String::from(path)})
        }

    }

    fn size(self: &Self) -> usize{
        return self.size;
    }

    fn insert<K, V>(self: &mut Self, key: K, value: V) -> std::io::Result<()>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::Serialize + Default + Debug{

        let key_text = serde_json::to_string(&key).unwrap();
        let value_text = serde_json::to_string(&value).unwrap();


        let sha_hash = digest(format!("{:?}",key));
        let sha_prefix = &sha_hash[0..9];
        // println!("hash of {:?} = {}", key, sha_hash);

        let dir_path = format!("{}/{}",self.path,sha_prefix);
        let key_path = format!("{}/{}.key",dir_path,sha_hash);
        let value_path = format!("{}/{}.value",dir_path,sha_hash);


        let f = File::open(&key_path);
        match f {
            Ok(_) => return Err(Error::new(ErrorKind::AlreadyExists, format!("There is already an entry with a key of {:?}",key))),
            Err(_) => (),
        };
        
        fs::create_dir_all(dir_path)?;
        fs::write(&key_path,&key_text).expect("Unable to write to key file");
        fs::write(&value_path,&value_text).expect("Unable to write to value file");
        self.size = self.size + 1;
        Ok(())
    }   

    fn lookup<K, V>(self: &Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug{
        let sha_hash = digest(format!("{:?}",key));
        let sha_prefix = &sha_hash[0..9];
        let value_path = format!("{}/{}/{}.value",self.path,sha_prefix,sha_hash);
        let f = File::open(&value_path);
        match f {
            Ok(_) => (),
            Err(_) => return Err(Error::new(ErrorKind::NotFound, format!("Couldn't find a value with that key."))),
        };
        let value_text = &fs::read_to_string(&value_path)?;
        let value_data:V = serde_json::from_str(&value_text).unwrap();
        Ok(value_data)

    }

    
    fn remove<K, V>(self: &mut Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug{
        let sha_hash = digest(format!("{:?}",key));
        let sha_prefix = &sha_hash[0..9];
        let dir_path = format!("{}/{}",self.path,sha_prefix);
        let key_path = format!("{}/{}/{}.key",self.path,sha_prefix,sha_hash);
        let value_path = format!("{}/{}/{}.value",self.path,sha_prefix,sha_hash);
        let value_data = self.lookup::<K,V>(key)?;

        fs::remove_file(key_path)?;
        fs::remove_file(value_path)?;
        self.size = self.size - 1;

        let paths = read_dir(&dir_path).unwrap();
        if paths.count() == 0 as usize{
            fs::remove_dir(dir_path)?;
        }

        Ok(value_data)
    }   
}

