use birb::Module;
use std::collections::HashMap;


use std::time::SystemTime;



pub enum LogCategory {
    INFO,
    DEBUG,
    WARN,
    ERROR
}
pub struct LogEntry {
    pub module_str: String,
    pub msg_str: String,
    pub log_str: String,
    pub category: LogCategory
}

pub type T_Listener = fn(&LogEntry);


#[derive(Default,Debug)]
pub struct Log{
    listeners: Vec<T_Listener>
}

impl Log {
    #[must_use]
    pub fn new() -> Self {
        let mut log = Self::default();
        return log;

    }

   fn translate_mod_uid<T>(&self,object: &T) -> String {
        return std::any::type_name::<T>().clone().to_string();
    } // temporary - will be changed as necessary later to reflect global module IDs, etc.

   pub fn notify_listeners(&self, log: &LogEntry) {
        for listener in self.listeners.iter() {
            listener(log);
        }
    }

    pub fn register_listener(&mut self, func: T_Listener) {
        self.listeners.push(func);
    }

    pub fn deregister_listener(&mut self, func: T_Listener) {
        self.listeners.retain(|x| *x != func);
    }
    pub fn info<T>(&self, object: &T, msg: String) {
        let timestamp: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!")
        };
        let module_name:String = self.translate_mod_uid(object).clone();
        let logstring = String::from(format!("[{timestamp}] [{module_name}] [INFO]: {msg}"));
        let log_entry = LogEntry {
            module_str : module_name,
            msg_str : msg,
            log_str : logstring,
            category : LogCategory::INFO
        };
        self.notify_listeners(&log_entry);
    }
}

impl Module for Log{}