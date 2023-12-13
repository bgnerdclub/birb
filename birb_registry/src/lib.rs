#![feature(const_trait_impl)]
#![feature(fs_try_exists)]

use birb::MainThreadModule;
use std::{collections::HashMap, io::prelude::*, fs::File, fs};
use std::fmt::{Debug, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Default, Debug)]
pub struct Registry {
    data: HashMap<String, Vec<u8>>,
    default_save_point: String
}

const SAVE_DEFAULT_LOCATION: &str = "registry_store.json";

// Implement Registry Methods
impl Registry {
    pub fn new() -> Self {
        return Registry {
            data: HashMap::default(),
            default_save_point: SAVE_DEFAULT_LOCATION.to_string()
        };
    }
    pub fn store<T>(&mut self, key: String, object: &T) where T: Serialize {
        self.data.insert(key, serde_json::to_string(object).unwrap().into_bytes());
    }
    pub fn get<T>(&self, key: String) -> T where T: for<'a> Deserialize<'a> {
        return serde_json::from_slice(self.data.get(&key).unwrap()).unwrap();
    }
    pub fn load(&mut self, save_point: Option<&str>) {
        if fs::try_exists(save_point.unwrap_or(&self.default_save_point)).unwrap() {
            let mut file = File::open(save_point.unwrap_or(&self.default_save_point)).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            self.data = serde_json::from_str(&data).unwrap();
        }
    }

    fn create_save_file(&mut self, save_point: Option<&str>) {
        File::create(save_point.unwrap_or(&self.default_save_point)).unwrap().write(serde_json::to_vec(&self.data).unwrap().as_slice()).unwrap();
    }
    pub fn save(&mut self, save_point: Option<&str>) {
        if fs::try_exists(save_point.unwrap_or(&self.default_save_point)).unwrap() {
            File::open(save_point.unwrap_or(&self.default_save_point)).unwrap().write(serde_json::to_vec(&self.data).unwrap().as_slice()).unwrap();
        } else {
            self.create_save_file(save_point)
        }
    }
}

impl MainThreadModule for Registry {}

#[cfg(test)]
mod tests {
    use crate::Registry;

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