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
    pub module_id: u16,
    pub module_str: String,
    pub msg_str: String,
    pub log_str: String,
    pub category: LogCategory
}

pub type T_Listener = fn(&LogEntry);


#[derive(Default,Debug)]
pub struct Log{
    uidmap: HashMap<u16,String>,
    listeners: Vec<T_Listener>
}

impl Log {
    #[must_use]
    pub fn new() -> Self {
        let mut log = Self::default();
        log.uidmap = HashMap::from([ // temporary - will be changed as necessary later to reflect global module IDs, etc.
            (0u16,String::from("core")),
            (1u16,String::from("winit")),
            (2u16,String::from("window")),
            (3u16,String::from("logs")),
            (4u16,String::from("other"))
        ]);
        return log;

    }

   fn translate_mod_uid(&self,uid: u16) -> &String {
        match self.uidmap.get(&uid) {
            Some(descriptor) => descriptor,
            None => &self.uidmap.get(&4u16).unwrap()
        }
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
    pub fn info(&self, module_uid: u16, msg: String) {
        let timestamp: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!")
        };
        let module_name:String = self.translate_mod_uid(module_uid).clone();
        let logstring = String::from(format!("[{timestamp}] [{module_name}] [INFO]: {msg}"));
        let log_entry = LogEntry {
            module_id : module_uid,
            module_str : module_name,
            msg_str : msg,
            log_str : logstring,
            category : LogCategory::INFO
        };
        self.notify_listeners(&log_entry);
    }
}

impl Module for Log{}