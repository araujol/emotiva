//! Easing Functions for Tweening Animations
//!
//! This module provides a collection of easing functions commonly used
//! in animation and tweening. These functions map a linear time value `t`
//! in the range [0.0, 1.0] to a non-linear curve that produces more natural
//! motion such as acceleration, deceleration, or bouncing.
//!
//! Supported easing types include:
//! - Linear
//! - Quadratic (In, Out, InOut)
//! - Cubic (In, Out, InOut)
//! - Sine (In, Out, InOut)
//!
//! These functions are pure and stateless, making them ideal for use in
//! animation engines, game loops, or real-time visual applications.

use serde::{Deserialize, Serialize};
use std::f32::consts::{FRAC_PI_2, PI};

/// Supported easing curve.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Easing {
    Linear,
    SineIn,
    SineOut,
    SineInOut,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
}

pub fn resolve(easing: Easing, t: f32) -> f32 {
    match easing {
        Easing::Linear => linear(t),
        Easing::SineIn => sine_in(t),
        Easing::SineOut => sine_out(t),
        Easing::SineInOut => sine_in_out(t),
        Easing::QuadIn => quad_in(t),
        Easing::QuadOut => quad_out(t),
        Easing::QuadInOut => quad_in_out(t),
        Easing::CubicIn => cubic_in(t),
        Easing::CubicOut => cubic_out(t),
        Easing::CubicInOut => cubic_in_out(t),
    }
}

/// The most honest easing function. Just goes from A to B without playing with your heart.
/// Produces constant speed with no acceleration or deceleration.
pub fn linear(t: f32) -> f32 {
    t
}

/// Starts fast and decelerates toward the end. Smooth and natural stopping motion.
pub fn cubic_out(t: f32) -> f32 {
    let p = t - 1.0;
    p * p * p + 1.0
}

/// Starts slow and accelerates over time. Great for launching motions.
pub fn cubic_in(t: f32) -> f32 {
    t * t * t
}

/// Combines cubic_in and cubic_out. Starts slow, speeds up, then slows again.
pub fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let p = 2.0 * t - 2.0;
        0.5 * p * p * p + 1.0
    }
}

/// Starts very slow and ramps up gently. Useful for smooth fade-ins.
pub fn sine_in(t: f32) -> f32 {
    1.0 - (t * FRAC_PI_2).cos()
}

/// Starts quickly and decelerates softly. Ideal for easing out motion.
pub fn sine_out(t: f32) -> f32 {
    (t * FRAC_PI_2).sin()
}

/// Combines sine_in and sine_out. Smooth acceleration and deceleration.
pub fn sine_in_out(t: f32) -> f32 {
    -0.5 * ((PI * t).cos() - 1.0)
}

/// Starts slow and speeds up linearly. A simple acceleration curve.
pub fn quad_in(t: f32) -> f32 {
    t * t
}

/// Starts fast and slows down linearly. Simple deceleration.
pub fn quad_out(t: f32) -> f32 {
    t * (2.0 - t)
}

/// Combines quad_in and quad_out. Accelerates then decelerates.
pub fn quad_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}
