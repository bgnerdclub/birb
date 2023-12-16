use birb::{App, Module, TypedEntityID};
use birb_maths::two::*;
use birb_transform::Transform;
use birb_utils::time::Clock;

pub struct Square {
    width: f32,
    height: f32,
    transform: TypedEntityID<Transform<f32>>,
}

#[derive(Debug)]
pub struct Spinner {
    velocity: f32,
}

impl Module for Spinner {
    fn tick(&mut self, app: &App) {
        let squares = app.get_entities_mut::<Square>().unwrap();
        let delta = app.get_module::<Clock>().unwrap().delta();
        let rotation = Rotor::from_angle(self.velocity * delta.as_secs_f32());

        for square in squares.iter() {
            let mut transform = app.get_entity_mut(square.transform).unwrap();
            transform.rotation *= rotation;
            println!("{:?}", transform.rotation);
        }
    }
}

pub fn main() {
    let mut app = App::new();
    app.register(Clock::register);
    app.register_module(Spinner { velocity: 1.0 });
    let transform = app.register_entity(Transform::identity(None));
    app.register_entity(Square {
        width: 1.0,
        height: 1.0,
        transform,
    });
    app.run();
}
