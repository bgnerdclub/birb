use birb::{App, Module};
use birb_window::{Key, Window};
use birb_winit::WinitWindow;

#[derive(Debug)]
struct CloseOnEscape {}

impl Module for CloseOnEscape {
    fn tick(&mut self, app: &mut App) {
        if app.get_module::<Window>().unwrap().is_down(Key::Escape) {
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
