use rayon::prelude::*;
use std::time::Instant;

use birb::{App, Module};
use birb_maths::two::*;
use birb_utils::time::Clock;

#[derive(Copy, Clone)]
struct Birb {
    position: Vector<f32>,
    velocity: Vector<f32>,
}

#[derive(Debug)]
struct BirbSystem {}

impl Module for BirbSystem {
    fn tick(&mut self, app: &App) {
        let gravity = app.get_module::<Gravity>().unwrap().gravity;
        let delta = app.get_module::<Clock>().unwrap().delta();
        app.get_entities_mut::<Birb>()
            .unwrap()
            .par_iter_mut()
            .for_each(|birb| {
                birb.position += birb.velocity * delta.as_secs_f32();
                birb.velocity += gravity * delta.as_secs_f32();
            })
    }
}

#[derive(Debug)]
struct Gravity {
    pub gravity: Vector<f32>,
}

impl Module for Gravity {
    fn tick(&mut self, _: &App) {}
}

pub fn main() {
    let mut app = App::new();
    app.register_module(BirbSystem {});
    app.register_module(Gravity {
        gravity: Vector::new(0.0, -9.81),
    });
    app.register(Clock::register);

    let birbs = vec![
        Birb {
            position: Vector::fill(0.0),
            velocity: Vector::fill(0.0)
        };
        1_000_000
    ];
    app.register_entities(&birbs);

    let start = Instant::now();
    for _ in 0..60 {
        app.tick()
    }
    let end = Instant::now();

    println!(
        "Fell to: {:?}",
        app.get_entities::<Birb>()
            .unwrap()
            .first()
            .unwrap()
            .position
    );

    println!(
        "1 tick took: {}ns {:.5}s",
        (end - start).as_nanos() / 60,
        (end - start).as_secs_f64() / 60.0
    );
}
