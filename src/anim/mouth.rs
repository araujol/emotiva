//! Emotiva Mouth Animation Subsystem
//!
//! This module provides procedural mouth animation for talking characters.
//! It defines a rhythmic, deterministic system where mouth flaps open and close
//! based on three configurable parameters:
//!
//! 🗣️ **Speaking Behavior Overview**
//!
//! A *talk session* begins at regular intervals, lasts for a fixed duration,
//! and during that time the mouth flaps (opens and closes) on a rhythm.
//!
//! - `talk_duration`: how long each talk session lasts (in seconds)
//! - `talk_interval`: how long to wait before the next session starts
//! - `flap_open_time`: how long the mouth stays open before closing
//!     - The full flap cycle is: `open → close`, so **1 flap = flap_open_time * 2**
//!
//! Example rhythm:
//! ```text
//! talk_duration = 1.6
//! flap_open_time = 0.4
//! → Flap cycle = 0.8s → 2 full flaps per session
//!
//! talk_interval = 0.4 → 400ms pause between each session
//! ```
//!
//! This system is deterministic, does not drift over time, and supports
//! fine-tuned rhythm styles: staccato, fluid, or dramatic — all by adjusting
//! the open time and duration.

use crate::AnimEvent;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MouthState {
    talking_enabled: bool,
    talking: bool,
    talk_duration: f32,
    talk_interval: f32,
    flap_open_time: f32,
    next_talk_time: f32,
    last_talk_started: f32,
    animation_id: Option<u64>,
}

impl MouthState {
    pub fn new() -> Self {
        Self {
            talking_enabled: false,
            talking: false,
            // TODO: Default values could go in a default module
            talk_duration: 1.2,
            talk_interval: 2.2,
            flap_open_time: 0.3,
            next_talk_time: 0.0,
            last_talk_started: 0.0,
            animation_id: None,
        }
    }

    pub fn with_config(talk_duration: f32, talk_interval: f32, flap_time: f32) -> Self {
        Self {
            talk_duration: talk_duration,
            talk_interval: talk_interval,
            flap_open_time: flap_time,
            ..Self::new()
        }
    }

    pub fn set_animation_id(&mut self, id: u64) {
        self.animation_id = Some(id);
    }

    // AnimEvent is returned per talk cycle (every flap begin and end)
    pub fn update(&mut self, time: f32, _rng: &mut impl Rng) -> AnimEvent {
        if !self.talking_enabled {
            return AnimEvent::None;
        }

        // Initialize first talk time if needed
        if self.next_talk_time == 0.0 {
            self.next_talk_time = time;
        }

        if self.talking {
            if time >= self.last_talk_started + self.talk_duration {
                self.talking = false;
                self.next_talk_time = time + self.talk_interval;
                return AnimEvent::Completed(self.animation_id);
            }
        } else if time >= self.next_talk_time {
            self.talking = true;
            self.last_talk_started = time;
            return AnimEvent::Started(self.animation_id);
        }

        AnimEvent::None
    }

    /// Returns whether the mouth should be open at the given time.
    ///
    /// When talking is active, the mouth flaps open and closed in a loop.
    /// Each flap is composed of an "open" phase followed by a "closed" phase,
    /// with each full cycle lasting `flap_open_time * 2.0` seconds.
    pub fn is_open(&self, time: f32) -> bool {
        if self.talking {
            let cycle = self.flap_open_time * 2.0;
            let phase = (time - self.last_talk_started) % cycle;
            phase < self.flap_open_time
        } else {
            false
        }
    }

    // Test if a talk session is active
    pub fn is_talking(&self) -> bool {
        self.talking
    }

    // Test is talk sessions are enabled to happen
    pub fn is_talking_enabled(&self) -> bool {
        self.talking_enabled
    }

    pub fn start(&mut self) {
        self.talking_enabled = true;
        self.next_talk_time = 0.0;
    }

    pub fn stop(&mut self) {
        self.talking_enabled = false;
        self.talking = false;
        self.next_talk_time = 0.0;
    }

    pub fn set_talk_interval(&mut self, interval: f32) {
        self.talk_interval = interval;
    }

    pub fn set_talk_duration(&mut self, duration: f32) {
        self.talk_duration = duration;
    }

    pub fn set_flap_open_time(&mut self, duration: f32) {
        self.flap_open_time = duration;
    }
}
