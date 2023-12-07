use std::time::Instant;

use birb::{App, Module};

struct Birb {
    id: u32,
}

#[derive(Debug)]
struct BirbSystem {}

impl Module for BirbSystem {
    fn tick(&mut self, app: &mut App) {
        let offset = app.get_module::<SystemTwo>().unwrap().offset;
        for birb in app.get_entity_mut::<Birb>() {
            birb.id += offset;
        }
    }
}

#[derive(Debug)]
struct SystemTwo {
    pub offset: u32,
}

impl Module for SystemTwo {
    fn tick(&mut self, _: &mut App) {}
}

pub fn main() {
    let mut app = App::new();
    app.register_module(BirbSystem {});
    app.register_module(SystemTwo { offset: 1 });

    for _ in 0..1_000_000 {
        app.register_entity(Birb { id: 0 })
    }

    let start = Instant::now();
    for _ in 0..60 {
        app.tick()
    }
    let end = Instant::now();

    println!("{}us", (end - start).as_micros());
}
