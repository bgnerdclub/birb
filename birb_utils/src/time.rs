use std::time::{Duration, Instant};

use birb::{App, Module};

#[derive(Debug)]
pub struct Clock {
    start: Instant,
    last_frame: Instant,
    delta: Duration,
}

impl Clock {
    pub fn register(app: &mut App) {
        let now = Instant::now();
        app.register_module(Self {
            start: now,
            last_frame: now,
            delta: Duration::ZERO,
        });
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }
}

impl Module for Clock {
    fn tick(&mut self, _: &App) {
        let now = Instant::now();
        self.delta = now - self.last_frame;
        self.last_frame = now;
    }
}
