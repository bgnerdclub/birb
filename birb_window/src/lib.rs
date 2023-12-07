#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use birb::Module;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Escape,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyPress(Key),
    KeyRelease(Key),
}

#[derive(Debug, Default)]
pub struct Window {
    down: Vec<Key>,
}

impl Window {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn is_down(&self, key: Key) -> bool {
        self.down.contains(&key)
    }

    pub fn submit(&mut self, event: Event) {
        println!("{event:?}");
        match event {
            Event::KeyPress(key) => {
                if !self.down.contains(&key) {
                    self.down.push(key);
                }
            }
            Event::KeyRelease(key) => {
                if let Some(index) = self.down.iter().position(|x| *x == key) {
                    self.down.remove(index);
                }
            }
        }
    }
}

impl Module for Window {}
