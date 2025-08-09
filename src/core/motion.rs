//! Emotiva Motion – Offset Animation Runtime Logic
//!
//! This module handles simple offset and rotation animations for character layers,
//! using the `Motion` struct from the `format` module. It enables
//! linear or eased transitions between two positions or angles over time,
//! such as smooth movement, pose shifts, or rotation transitions during scene changes.

use crate::core::easing::{Easing, resolve};
use crate::core::events::AnimEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Reverse,
}

#[derive(Debug, Clone)]
pub struct Motion2D {
    start: (f32, f32),
    end: (f32, f32),
    duration: f32,
    elapsed: f32,
    playing: bool,
    direction: Direction,
    easing: Easing,
    animation_id: Option<u64>, // unique animation identifier
}

impl Motion2D {
    pub fn new(start: (f32, f32), end: (f32, f32), duration: f32, easing: Easing) -> Self {
        Self {
            start,
            end,
            duration,
            elapsed: 0.0,
            playing: false,
            direction: Direction::Forward,
            easing,
            animation_id: None,
        }
    }

    /// Assign an animation ID for this Motion2D.
    pub fn set_animation_id(&mut self, id: u64) {
        self.animation_id = Some(id);
    }

    pub fn play(&mut self) {
        self.elapsed = 0.0;
        self.direction = Direction::Forward;
        self.playing = true;
    }

    pub fn reverse(&mut self) {
        self.elapsed = 0.0;
        self.direction = Direction::Reverse;
        self.playing = true;
    }

    pub fn update(&mut self, dt: f32) -> AnimEvent {
        // If not playing, no state change.
        if !self.playing {
            return AnimEvent::None;
        }

        // First frame logic
        if self.elapsed == 0.0 {
            // Increment time but still return Started this frame.
            self.elapsed += dt;
            if self.elapsed >= self.duration {
                self.elapsed = self.duration;
                self.playing = false;
                return AnimEvent::Completed(self.animation_id);
            }
            return AnimEvent::Started(self.animation_id);
        }

        // Continue running.
        self.elapsed += dt;

        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.playing = false;
            AnimEvent::Completed(self.animation_id)
        } else {
            AnimEvent::None
        }
    }

    pub fn value(&self) -> (f32, f32) {
        let t = (self.elapsed / self.duration).clamp(0.0, 1.0);
        let eased = resolve(self.easing, t);

        match self.direction {
            Direction::Forward => (
                self.start.0 + (self.end.0 - self.start.0) * eased,
                self.start.1 + (self.end.1 - self.start.1) * eased,
            ),
            Direction::Reverse => (
                self.end.0 + (self.start.0 - self.end.0) * eased,
                self.end.1 + (self.start.1 - self.end.1) * eased,
            ),
        }
    }

    pub fn is_finished(&self) -> bool {
        !self.playing
    }
}

#[derive(Debug, Clone)]
pub struct Rotation {
    motion: Motion2D,
}

impl Rotation {
    pub fn new(degrees: f32, duration: f32, easing: Easing) -> Self {
        Self {
            motion: Motion2D::new((0.0, 0.0), (degrees.to_radians(), 0.0), duration, easing),
        }
    }

    /// Assign an animation ID for this Rotation's Motion2D.
    pub fn set_animation_id(&mut self, id: u64) {
        self.motion.set_animation_id(id);
    }

    pub fn update(&mut self, dt: f32) -> AnimEvent {
        self.motion.update(dt)
    }

    pub fn play(&mut self) {
        self.motion.play()
    }

    pub fn reverse(&mut self) {
        self.motion.reverse()
    }

    pub fn value(&self) -> f32 {
        self.motion.value().0
    }

    pub fn is_finished(&self) -> bool {
        self.motion.is_finished()
    }
}
