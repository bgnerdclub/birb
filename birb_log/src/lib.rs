use birb::Module;
use std::time::SystemTime;

pub enum LogCategory {
    INFO,
    DEBUG,
    WARN,
    ERROR,
}
pub struct LogEntry {
    pub module: String,
    pub msg: String,
    pub timestamp: u64,
    pub category: LogCategory,
}

pub type Listener = fn(&LogEntry);

#[derive(Default, Debug)]
pub struct Log {
    listeners: Vec<Listener>,
}

impl Log {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn translate_mod_uid<T>(&self, _object: &T) -> String {
        std::any::type_name::<T>().to_string()
    } // temporary - will be changed as necessary later to reflect global module IDs, etc.

    pub fn notify_listeners(&self, log: &LogEntry) {
        for listener in &self.listeners {
            listener(log);
        }
    }

    pub fn register_listener(&mut self, func: Listener) {
        self.listeners.push(func);
    }

    pub fn deregister_listener(&mut self, func: Listener) {
        self.listeners.retain(|x| *x != func);
    }
    pub fn info<T>(&self, object: &T, message: String) {
        let ts: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before unix epoch!")
            .as_secs();
        let module_name: String = self.translate_mod_uid(object);
        let log_entry = LogEntry {
            module: module_name,
            msg: message,
            timestamp: ts,
            category: LogCategory::INFO,
        };
        self.notify_listeners(&log_entry);
    }
}

impl Module for Log {}
