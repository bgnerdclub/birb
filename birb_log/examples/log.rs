use birb_log;
use birb::{App, Module};

#[derive(Debug)]
struct LogExample {}

impl Module for LogExample {
    fn tick(&mut self, app: &App) {
        let log_module = app.get_module::<birb_log::Log>().unwrap();
        log_module.info(&self, String::from("Test message"));
    }
}

pub fn test_listener(log_entry: &birb_log::LogEntry) {
    println!("{}", log_entry.log_str)
}

pub fn main() {
    let mut app = App::new();
    let mut log = birb_log::Log::new();
    let listen_ptr: birb_log::T_Listener = test_listener;
    log.register_listener(listen_ptr);
    app.register_module(log);
    app.register_module(LogExample {});
    app.run();
}