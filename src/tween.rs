//! Emotiva Tween – Layer Motion Runtime Logic
//!
//! This module provides basic runtime tweening (sway/bobbing) for character layers
//! based on the data defined in the `Tween` struct from the `format` module.

use crate::format::Tween;
use std::f32::consts::PI;

/// Represents the animated state of a tweened layer at runtime.
#[derive(Debug, Default, Clone, Copy)]
pub struct TweenState {
    pub time: f32,
}

/// The current per-frame animated offset for a layer.
#[derive(Debug, Clone, Copy)]
pub struct TweenOffset {
    pub dx: f32,
    pub dy: f32,
}

impl TweenOffset {
    pub fn zero() -> Self {
        Self { dx: 0.0, dy: 0.0 }
    }
}

impl TweenState {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }

    /// Advance time and compute the current offset for a given sway definition.
    pub fn update(&mut self, dt: f32, tween: &Tween) -> TweenOffset {
        self.time += dt;

        let phase = (self.time * 2.0 * PI) / tween.period;
        let sin = phase.sin();

        TweenOffset {
            dx: tween.sway.0 * sin,
            dy: tween.sway.1 * sin,
        }
    }
}
