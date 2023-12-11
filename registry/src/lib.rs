#![feature(const_trait_impl)]

use birb::Module;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default)]
pub struct Registry {
    data: HashMap<String, Vec<u8>>
}

// Implement Registry Methods
impl Registry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn store<T>(&mut self, key: String, object: &T) where T: Serialize {
        let jstring = serde_json::to_string(object);
        self.data.insert(key, jstring.unwrap().into_bytes());
    }
    pub fn get<T>(&self, key: String) -> T where T: for<'a> Deserialize<'a> {
        let jstring = self.data.get(&key).unwrap();
        let object: T = serde_json::from_slice(jstring).unwrap();
        return object
    }
}

impl Module for Registry {}

#[test]
fn test_registry_kv() {
    let mut registry = Registry::new();
    const INDEX_VALUE: &str = "test_value";
    const INDEX_KEY: &str = "TEST/key";
    registry.store::<String>(INDEX_KEY.to_string(), &INDEX_VALUE.to_string());
    let rsval = registry.get::<String>(INDEX_KEY.to_string());
    assert_eq!(&rsval, &INDEX_VALUE);
}