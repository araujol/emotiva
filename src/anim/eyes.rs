//! Emotiva Eyes Animation Subsystem
//!
//! This module provides procedural animation logic for eye blinking,
//! including open/closed transitions using randomized timing.
//! It simulates natural blinking cycles to enhance character realism.
//!
//! Used by: `CharAnimator` to control which eye sprite is shown each frame.

use rand::Rng;

pub struct EyesState {
    next_blink_time: f32,
    blink_duration: f32,
    blinking: bool,
}

impl EyesState {
    pub fn new(current_time: f32, rng: &mut impl Rng) -> Self {
        // Change this range to modify how often blinking starts (in seconds)
        let next = current_time + rng.random_range(3.0..6.0);
        Self {
            next_blink_time: next,
            blink_duration: 0.12, // Change this value to modify how long a blink lasts
            blinking: false,
        }
    }

    pub fn update(&mut self, time: f32, rng: &mut impl Rng) {
        if self.blinking {
            if time >= self.next_blink_time + self.blink_duration {
                self.blinking = false;
                // Reset the next blink time after the current blink ends
                self.next_blink_time = time + rng.random_range(3.0..6.0);
            }
        } else if time >= self.next_blink_time {
            // Start a new blink when the scheduled time is reached
            self.blinking = true;
        }
    }

    pub fn is_blinking(&self) -> bool {
        self.blinking
    }
}
