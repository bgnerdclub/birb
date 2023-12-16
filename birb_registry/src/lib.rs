use birb::Module;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::Path;
use std::{collections::HashMap, fs::File, io::prelude::*};

#[derive(Default, Debug)]
pub struct Registry {
    data: HashMap<String, Vec<u8>>,
}

// Implement Registry Methods
impl Registry {
    pub fn new() -> Self {
        Registry {
            data: HashMap::default(),
        }
    }
    pub fn store<T>(&mut self, key: String, object: &T)
    where
        T: Serialize,
    {
        self.data
            .insert(key, serde_json::to_string(object).unwrap().into_bytes());
    }
    pub fn get<T>(&self, key: String) -> T
    where
        T: for<'a> Deserialize<'a>,
    {
        return serde_json::from_slice(self.data.get(&key).unwrap()).unwrap();
    }
    pub fn load<T: AsRef<Path>>(&mut self, path: T) {
        let path = path.as_ref();
        if path.exists() {
            let mut file = File::open(path).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            self.data = serde_json::from_str(&data).unwrap();
        }
    }

    fn create_save_file<T: AsRef<Path>>(&mut self, path: T) {
        File::create(path.as_ref())
            .unwrap()
            .write_all(serde_json::to_vec(&self.data).unwrap().as_slice())
            .unwrap();
    }

    pub fn save<T: AsRef<Path>>(&mut self, path: T) {
        let path = path.as_ref();
        if path.exists() {
            File::open(path)
                .unwrap()
                .write_all(serde_json::to_vec(&self.data).unwrap().as_slice())
                .unwrap();
        } else {
            self.create_save_file(path)
        }
    }
}

impl Module for Registry {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_registry_kv() {
        let mut registry = Registry::new();
        const INDEX_VALUE: &str = "test_value";
        const INDEX_KEY: &str = "TEST/key";
        registry.store::<String>(INDEX_KEY.to_string(), &INDEX_VALUE.to_string());
        let return_value = registry.get::<String>(INDEX_KEY.to_string());
        assert_eq!(&return_value, &INDEX_VALUE);
    }
}

