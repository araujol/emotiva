//! Emotiva Eyes Animation Subsystem
//!
//! This module provides procedural animation logic for eye blinking,
//! including open/closed transitions using randomized timing.
//! It simulates natural blinking cycles to enhance character realism.
//!
//! Used by: `EmotivaHeart` to control which eye sprite is shown each frame.

use crate::AnimEvent;
use rand::Rng;

pub struct EyesState {
    next_blink_time: f32,
    blink_duration: f32,
    blinking: bool,
    blinking_enabled: bool,
    blink_interval_range: (f32, f32),
    animation_id: Option<u64>,
}

impl EyesState {
    pub fn new() -> Self {
        // Change this range to modify how often blinking starts (in seconds)
        let interval_range = (3.0, 6.0);
        Self {
            next_blink_time: 0.0,
            blink_duration: 0.12, // Change this value to modify how long a blink lasts
            blinking: false,
            blinking_enabled: false,
            blink_interval_range: interval_range,
            animation_id: None,
        }
    }

    pub fn with_config(interval_range: (f32, f32), duration: f32) -> Self {
        Self {
            next_blink_time: 0.0,
            blink_duration: duration,
            blinking: false,
            blinking_enabled: false,
            blink_interval_range: interval_range,
            animation_id: None,
        }
    }

    pub fn set_animation_id(&mut self, id: u64) {
        self.animation_id = Some(id);
    }

    pub fn update(&mut self, time: f32, rng: &mut impl Rng) -> AnimEvent {
        if !self.blinking_enabled {
            return AnimEvent::None;
        }

        // Initialize next blinking time
        if self.next_blink_time == 0.0 {
            self.next_blink_time = time + Self::safe_gen_range(rng, self.blink_interval_range);
        }

        if self.blinking {
            if time >= self.next_blink_time + self.blink_duration {
                self.blinking = false;
                // Reset the next blink time after the current blink ends
                self.next_blink_time = time + Self::safe_gen_range(rng, self.blink_interval_range);
                return AnimEvent::Completed(self.animation_id);
            }
        } else if time >= self.next_blink_time {
            // Start a new blink when the scheduled time is reached
            self.blinking = true;
            return AnimEvent::Started(self.animation_id);
        }

        AnimEvent::None
    }

    pub fn is_blinking(&self) -> bool {
        self.blinking
    }

    pub fn is_blinking_active(&self) -> bool {
        self.blinking_enabled
    }

    pub fn start(&mut self) {
        self.blinking_enabled = true;
        self.next_blink_time = 0.0;
    }

    pub fn stop(&mut self) {
        self.blinking_enabled = false;
        self.blinking = false; // ensure eyes are OPEN
        self.next_blink_time = 0.0;
    }

    pub fn set_interval_range(&mut self, range: (f32, f32)) {
        self.blink_interval_range = range;
    }

    pub fn set_blink_duration(&mut self, duration: f32) {
        self.blink_duration = duration;
    }

    fn safe_gen_range(rng: &mut impl Rng, range: (f32, f32)) -> f32 {
        if (range.0 - range.1).abs() < f32::EPSILON {
            range.0
        } else {
            rng.random_range(range.0..range.1)
        }
    }
}
