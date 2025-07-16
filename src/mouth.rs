// emotiva/src/mouth.rs

use rand::Rng;

#[derive(Debug)]
pub struct MouthState {
    pub is_talking: bool,
    pub next_phase_time: f32,
}

impl MouthState {
    pub fn new(initial_time: f32, rng: &mut impl Rng) -> Self {
        let talking = false;
        let duration = rng.random_range(2.0..=5.0); // initial silence duration
        Self {
            is_talking: talking,
            next_phase_time: initial_time + duration,
        }
    }

    pub fn update(&mut self, time: f32, rng: &mut impl Rng) {
        if time >= self.next_phase_time {
            self.is_talking = !self.is_talking;
            let duration = if self.is_talking {
                rng.random_range(0.5..=2.0) // talking duration
            } else {
                rng.random_range(2.0..=5.0) // silence duration
            };
            self.next_phase_time = time + duration;
        }
    }

    pub fn is_open(&self, time: f32) -> bool {
        if self.is_talking {
            let cycle = 0.4;
            (time % cycle) < (cycle / 2.0)
        } else {
            false
        }
    }
}
