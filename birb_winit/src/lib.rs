#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use birb::{App, MainThreadApp, MainThreadModule, Module};
use birb_window::{Event, Key};

use crossbeam_channel::{unbounded, Receiver, Sender};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::{event, platform::x11::EventLoopBuilderExtX11};

#[allow(clippy::too_many_lines)]
const fn winit_to_key(key: winit::keyboard::PhysicalKey) -> Option<Key> {
    match key {
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) => Some(Key::Escape),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F1) => Some(Key::F1),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F2) => Some(Key::F2),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F3) => Some(Key::F3),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F4) => Some(Key::F4),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F5) => Some(Key::F5),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F6) => Some(Key::F6),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F7) => Some(Key::F7),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F8) => Some(Key::F8),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F9) => Some(Key::F9),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F10) => Some(Key::F10),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F11) => Some(Key::F11),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F12) => Some(Key::F12),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Backquote) => {
            Some(Key::Backquote)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit1) => Some(Key::Key1),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit2) => Some(Key::Key2),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit3) => Some(Key::Key3),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit4) => Some(Key::Key4),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit5) => Some(Key::Key5),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit6) => Some(Key::Key6),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit7) => Some(Key::Key7),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit8) => Some(Key::Key8),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit9) => Some(Key::Key9),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit0) => Some(Key::Key0),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Minus) => Some(Key::Hyphen),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Equal) => Some(Key::Equals),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Tab) => Some(Key::Tab),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA) => Some(Key::A),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyB) => Some(Key::B),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyC) => Some(Key::C),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD) => Some(Key::D),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyE) => Some(Key::E),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyF) => Some(Key::F),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyG) => Some(Key::G),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyH) => Some(Key::H),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyI) => Some(Key::I),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyJ) => Some(Key::J),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyK) => Some(Key::K),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyL) => Some(Key::L),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyM) => Some(Key::M),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyN) => Some(Key::N),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyO) => Some(Key::O),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyP) => Some(Key::P),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyQ) => Some(Key::Q),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyR) => Some(Key::R),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS) => Some(Key::S),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyT) => Some(Key::T),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyU) => Some(Key::U),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyV) => Some(Key::V),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW) => Some(Key::W),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyX) => Some(Key::X),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyY) => Some(Key::Y),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyZ) => Some(Key::Z),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::BracketLeft) => {
            Some(Key::LeftSquare)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::BracketRight) => {
            Some(Key::RightSquare)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::CapsLock) => {
            Some(Key::CapsLock)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Semicolon) => {
            Some(Key::Semicolon)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Quote) => {
            Some(Key::Apostrophe)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::NumpadHash) => Some(Key::Hash),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Backslash) => {
            Some(Key::Backslash)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Period) => Some(Key::Period),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Comma) => Some(Key::Comma),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Slash) => Some(Key::Slash),

        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ShiftLeft) => {
            Some(Key::LeftShift)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ShiftRight) => {
            Some(Key::RightShift)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ControlLeft) => {
            Some(Key::LeftControl)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ControlRight) => {
            Some(Key::RightControl)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::SuperLeft) => {
            Some(Key::LeftSuper)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::SuperRight) => {
            Some(Key::RightSuper)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::AltLeft) => Some(Key::LeftAlt),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::AltRight) => {
            Some(Key::RightAlt)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowUp) => Some(Key::Up),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowDown) => Some(Key::Down),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft) => Some(Key::Left),
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowRight) => {
            Some(Key::Right)
        }
        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Space) => Some(Key::Space),
        _ => None,
    }
}

#[derive(Debug)]
pub struct WinitWindow {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
}

impl WinitWindow {
    /// # Panics
    /// Panics if window initialisation fails
    pub fn register(app: &mut MainThreadApp) {
        app.register_module(birb_window::Window::new());

        let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        app.register_main_thread_module(Self { event_loop, window });
    }

    pub fn run(
        event: winit::event::Event<()>,
        elwt: &winit::event_loop::EventLoopWindowTarget<()>,
    ) {
    }
}

impl MainThreadModule for WinitWindow {
    fn tick(&mut self, app: &MainThreadApp) {
        let mut window = app.get_module_mut::<birb_window::Window>().unwrap();
        self.event_loop
            .run_on_demand(|event, elwt| {
                elwt.exit();
                if let winit::event::Event::WindowEvent { event, .. } = event {
                    match event {
                        winit::event::WindowEvent::KeyboardInput { event, .. } => {
                            let Some(key) = winit_to_key(event.physical_key) else {
                                return;
                            };

                            match event.state {
                                winit::event::ElementState::Pressed => {
                                    window.submit(Event::KeyPress(key));
                                }
                                winit::event::ElementState::Released => {
                                    window.submit(Event::KeyRelease(key));
                                }
                            }
                        }
                        _ => (),
                    }
                }
            })
            .unwrap();
    }
}
