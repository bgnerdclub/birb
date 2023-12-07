use rayon::prelude::*;
use std::time::Instant;

use birb::{App, Module};

#[derive(Copy, Clone)]
struct Birb {
    id: u32,
}

#[derive(Debug)]
struct BirbSystem {}

impl Module for BirbSystem {
    fn tick(&mut self, app: &mut App) {
        let offset = app.get_module::<SystemTwo>().unwrap().offset;
        app.get_entity_mut::<Birb>()
            .par_iter_mut()
            .for_each(|birb| birb.id += offset)
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

    let birbs = [Birb { id: 0 }; 1_000_000];
    app.register_entities(&birbs);

    let start = Instant::now();
    for _ in 0..60 {
        app.tick()
    }
    let end = Instant::now();

    println!("{}us", (end - start).as_micros());
}
