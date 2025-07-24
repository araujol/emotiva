//! Emotiva Tween – Layer Sway & Idle Animation Runtime
//!
//! This module provides ambient tweening effects (like sway, bob, or lean)
//! for character layers, based on the `Tween` data defined in the `format` module.
//! These animations loop continuously and add natural motion to static images.

use crate::easing::{Easing, resolve};
use crate::format::Tween;

/// Represents the animated state of a tweened layer at runtime.
#[derive(Debug, Clone, Copy)]
pub struct TweenState {
    time: f32,
    enabled: bool,
    paused: bool,                               // whether this tween is paused
    ease_in_state: Option<(f32, TweenOffset)>,  // (progress, target offset)
    ease_out_state: Option<(f32, TweenOffset)>, // (progress, starting offset)
}

impl Default for TweenState {
    fn default() -> Self {
        Self {
            time: 0.0,
            enabled: false,
            paused: false,
            ease_in_state: None,
            ease_out_state: None,
        }
    }
}

/// The current per-frame animated offset for a layer.
#[derive(Debug, Clone, Copy)]
pub struct TweenOffset {
    pub dx: f32,
    pub dy: f32,
    pub rotation: f32, // in radians
}

impl TweenOffset {
    pub fn zero() -> Self {
        Self {
            dx: 0.0,
            dy: 0.0,
            rotation: 0.0,
        }
    }

    /// Linearly interpolates between the current TweenOffset (`self`) and a target offset,
    /// based on a given interpolation factor `t`.
    /// This is used to animate movement and rotation between two states over time.
    pub fn lerp(self, target: TweenOffset, t: f32) -> Self {
        Self {
            dx: self.dx + (target.dx - self.dx) * t,
            dy: self.dy + (target.dy - self.dy) * t,
            rotation: self.rotation + (target.rotation - self.rotation) * t,
        }
    }
}

impl TweenState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.enabled = true;
        self.paused = false;
    }

    pub fn stop(&mut self) {
        self.enabled = false;
        self.paused = false;
        self.time = 0.0;
        self.ease_in_state = None;
        self.ease_out_state = None;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn is_enabled(&mut self) -> bool {
        return self.enabled;
    }

    pub fn is_paused(&mut self) -> bool {
        return self.paused;
    }

    /// Start animation with easing in.
    pub fn start_easing(&mut self, _tween: &Tween) {
        self.time = 0.0;
        self.enabled = false; // becomes true after easing
        self.paused = false;
        self.ease_in_state = Some((0.0, TweenOffset::zero())); // always start from zero
    }

    /// Manually stop animation with easing out.
    pub fn stop_easing(&mut self, tween: &Tween) {
        let current_offset = self.compute_offset(tween);
        self.enabled = false;
        self.paused = false;
        self.ease_out_state = Some((0.0, current_offset));
        self.time = 0.0;
    }

    /// Compute current offset for the current time
    fn compute_offset(&self, tween: &Tween) -> TweenOffset {
        let phase = (self.time * 2.0 * std::f32::consts::PI) / tween.period;
        let sin = phase.sin();

        let mut offset = TweenOffset {
            dx: tween.sway.0 * sin,
            dy: tween.sway.1 * sin,
            rotation: 0.0,
        };

        if let Some(lean) = &tween.lean {
            let angle_deg = lean.max_angle * sin;
            offset.rotation = angle_deg.to_radians();
        }

        offset
    }

    /// Advance time and compute the current offset for a given sway definition.
    pub fn update(&mut self, dt: f32, tween: &Tween) -> TweenOffset {
        // Return the current offset if paused.
        if self.paused {
            return self.compute_offset(tween);
        }

        if let Some((ref mut start_time, start_target)) = self.ease_in_state {
            let easing_duration = 1.0;
            *start_time += dt / easing_duration;
            let t = (*start_time).min(1.0);
            if t >= 1.0 {
                self.ease_in_state = None;
                self.enabled = true;
            }
            let t_eased = resolve(tween.easing_start.unwrap_or(Easing::SineIn), t);
            TweenOffset::zero().lerp(start_target, t_eased)
        } else if self.enabled {
            self.time += dt;
            self.compute_offset(tween)
        } else if let Some((ref mut easing_time, start_offset)) = self.ease_out_state {
            let easing_duration = 1.0;
            *easing_time += dt / easing_duration;
            let t = (*easing_time).min(1.0);
            if t >= 1.0 {
                self.ease_out_state = None;
            }
            let t_eased = resolve(tween.easing_stop.unwrap_or(Easing::SineOut), t);
            start_offset.lerp(TweenOffset::zero(), t_eased)
        } else {
            TweenOffset::zero()
        }
    }
}
