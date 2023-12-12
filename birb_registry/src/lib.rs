#![feature(const_trait_impl)]
#![feature(fs_try_exists)]

use birb::MainThreadModule;
use std::{collections::HashMap, io::prelude::*, fs::File, fs};
use std::fmt::{Debug, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct Registry {
    ticks: u16,
    data: HashMap<String, Vec<u8>>
}

// Implement Registry Methods
impl Registry {
    pub fn new() -> Self {
        Registry::default()
    }
    pub fn store<T>(&mut self, key: String, object: &T) where T: Serialize {
        self.data.insert(key, serde_json::to_string(object).unwrap().into_bytes());
    }
    pub fn get<T>(&self, key: String) -> T where T: for<'a> Deserialize<'a> {
        let object: T = serde_json::from_slice(self.data.get(&key).unwrap()).unwrap();
        return object
    }
}

impl Debug for Registry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry")
            .field("data", &self.data)
            .finish()
    }
}

impl MainThreadModule for Registry {
    fn tick(&mut self, _: &birb::MainThreadApp) {
        // shitty persistence :3
        // loads json on zero
        if self.ticks == 0 {
            if fs::try_exists("birb_registry.json").unwrap() {
                match File::open("birb_registry.json") {
                    Ok(mut File) => {
                        let mut data = String::new();
                        match File.read_to_string(&mut data) {
                            Ok(_) => {
                                self.data = serde_json::from_str(&data).unwrap()
                            }
                            Err(_) => {
                                fs::remove_file("birb_registry.json").unwrap();
                                self.ticks = 0;
                                // early return to escape ++ increment on internal tick counter.
                                return
                            }
                        }
                    }
                    Err(E) => {
                        panic!("error in loading persistence \n {}", E);
                    }
                }
            } else {
                match File::create("birb_registry.json") {
                    Ok(mut File) => {
                        // create from empty Hashmap Array
                        match File.write(serde_json::to_vec(&self.data).unwrap().as_slice()) {
                            Err(E) => {
                                panic!("error saving persistence \n {}", E)
                            }
                            _ => {
                                // discard, dont care about OK() result
                            }
                        }
                    }
                    Err (E) => {
                        panic!("error in creating persistence \n {}", E);
                    }
                }
            }
        } else {
            // every 5 seconds
            if self.ticks > 300 {
                match File::open("birb_registry.json") {
                    Ok(mut File) => {
                        match File.write(serde_json::to_vec(&self.data).unwrap().as_slice()) {
                            Err (E) => {
                                panic!("error in persisting persistence \n {}", E);
                            }
                            _ => {
                                // discard, dont care about OK() result
                            }
                        }
                    }
                    Err(E) => {
                        panic!("error in persisting persistence \n {}", E);
                    }
                }
                self.ticks = 0;
            }
        }
        self.ticks += 1;
    }
}

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