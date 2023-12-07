use birb::{App, Module};
use birb_window::{Event, Key};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;

fn winit_to_key(key: winit::keyboard::PhysicalKey) -> Option<Key> {
    match key {
        winit::keyboard::PhysicalKey::Code(code) => match code {
            winit::keyboard::KeyCode::Escape => Some(Key::Escape),
            _ => None,
        },
        _ => None,
    }
}

#[derive(Debug)]
pub struct Window {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
}

impl Window {
    pub fn new(app: &mut App) {
        app.register_module(birb_window::Window::new());

        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        app.register_module(Self { event_loop, window })
    }
}

impl Module for Window {
    fn tick(&mut self, app: &mut App) {
        let mut window = app.get_module_mut::<birb_window::Window>().unwrap();
        self.event_loop
            .run_on_demand(|event, ewlt| {
                match event {
                    winit::event::Event::WindowEvent { event, .. } => match event {
                        winit::event::WindowEvent::KeyboardInput { event, .. } => {
                            let Some(key) = winit_to_key(event.physical_key) else {
                                return;
                            };

                            match event.state {
                                winit::event::ElementState::Pressed => {
                                    window.submit(Event::KeyPress(key))
                                }
                                winit::event::ElementState::Released => {
                                    window.submit(Event::KeyRelease(key))
                                }
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
                ewlt.exit()
            })
            .unwrap()
    }
}