use birb::Module;

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Escape,
}

#[derive(Debug, Clone)]
pub enum Event {
    KeyPress(Key),
    KeyRelease(Key),
}

#[derive(Debug)]
pub struct Window {
    down: Vec<Key>,
}

impl Window {
    pub fn new() -> Self {
        Self { down: Vec::new() }
    }

    pub fn is_down(&self, key: Key) -> bool {
        self.down.contains(&key)
    }

    pub fn submit(&mut self, event: Event) {
        println!("{:?}", event);
        match event {
            Event::KeyPress(key) => {
                if !self.down.contains(&key) { self.down.push(key) }
            }
            Event::KeyRelease(key) => match self.down.iter().position(|x| *x == key) {
                Some(index) => { self.down.remove(index); },
                None => ()
            }
        }
    }
}

impl Module for Window {}
