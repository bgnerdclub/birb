use birb::{App, Module};

#[derive(Debug)]
struct LogExample {}

impl Module for LogExample {
    fn tick(&mut self, app: &App) {
        let mut log_module = app.get_module_mut::<birb_log::Log>().unwrap();
        let listen_ptr: birb_log::Listener = test_listener;
        let listen_ptr2: birb_log::Listener = test_listener2;
        log_module.register_listener(listen_ptr);
        log_module.info(&self, String::from("Test message"));
        log_module.deregister_listener(listen_ptr);
        log_module.register_listener(listen_ptr2);
        log_module.info(&self, String::from("this string doesn't matter"));
        app.exit();
    }
}

pub fn test_listener(log_entry: &birb_log::LogEntry) {
    println!(
        "{} {} {}",
        log_entry.module, log_entry.timestamp, log_entry.msg
    );
}

pub fn test_listener2(_log_entry: &birb_log::LogEntry) {
    println!("i am test listener 2");
}

pub fn main() {
    let mut app = App::new();
    app.register_module(birb_log::Log::new());
    app.register_module(LogExample {});
    app.run();
}
