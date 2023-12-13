use birb::{App, Module};
use birb_window::{Key, Window};
use birb_winit::WinitWindow;

#[derive(Debug)]
struct CloseOnEscape {}

impl Module for CloseOnEscape {
    fn tick(&mut self, app: &App) {
        let window = app.get_module::<Window>().unwrap();
        if window.is_down(Key::Escape) || window.is_down(Key::Q) {
            println!("Exitting");
            app.exit()
        }
    }
}

pub fn main() {
    let mut app = App::new();
    app.register(WinitWindow::register);
    app.register_module(CloseOnEscape {});
    app.run();
}
