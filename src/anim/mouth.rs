//! Emotiva Mouth Animation Subsystem
//!
//! This module provides procedural animation logic for mouth movement,
//! including talking cycles and open/closed state transitions based on time.
//! It simulates idle speech patterns using randomness to vary timing.
//!
//! Used by: `CharAnimator` to control which mouth sprite is shown each frame.

use rand::Rng;

#[derive(Debug)]
pub struct MouthState {
    is_talking: bool,
    next_phase_time: f32,
    idle_talking: bool,
}

impl MouthState {
    pub fn new(initial_time: f32, rng: &mut impl Rng) -> Self {
        // Change the initial silence duration range here
        let duration = rng.random_range(2.0..=5.0); // initial silence duration
        Self {
            is_talking: false,
            next_phase_time: initial_time + duration,
            idle_talking: false,
        }
    }

    pub fn update(&mut self, time: f32, rng: &mut impl Rng) {
        // This only affects the idle talk option.
        if self.idle_talking {
            if time >= self.next_phase_time {
                self.is_talking = !self.is_talking;
                let duration = if self.is_talking {
                    // Change the talking duration range here
                    rng.random_range(0.5..=2.0) // talking duration
                } else {
                    // Change the silence duration range here
                    rng.random_range(2.0..=5.0) // silence duration
                };
                self.next_phase_time = time + duration;
            }
        }
    }

    pub fn is_open(&self, time: f32) -> bool {
        if self.is_talking {
            // Change the open/close cycle timing here
            let cycle = 0.4;
            (time % cycle) < (cycle / 2.0)
        } else {
            false
        }
    }

    pub fn start(&mut self) {
        self.is_talking = true;
    }

    pub fn stop(&mut self) {
        self.is_talking = false;
        self.idle_talking = false;
    }

    pub fn idle_chat(&mut self) {
        self.is_talking = true;
        self.idle_talking = true;
    }
}
