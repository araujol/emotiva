//! Emotiva Motion – Offset Animation Runtime Logic
//!
//! This module handles simple offset animations for character layers,
//! using the `Motion` struct from the `format` module. It enables
//! linear or eased transitions between two positions over time,
//! such as smooth movement or pose shifts during scene transitions.

use crate::easing::{Easing, resolve};

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
        }
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

    pub fn update(&mut self, dt: f32) {
        if !self.playing {
            return;
        }

        self.elapsed += dt;

        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.playing = false;
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
