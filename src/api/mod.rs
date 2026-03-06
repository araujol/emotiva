//! # 🎭 Emotiva API
//!
//! Main entry point for the **public Emotiva control API**.
//!
//! This module organizes and exposes the high-level runtime controls used
//! to animate and manipulate characters through [`EmotivaHeart`].
//!
//! Each submodule groups related animation systems into focused APIs
//! (motions, tweens, FX, behaviors, etc.), while the [`EmotivaAPI`] trait
//! defines the unified interface exposed to frontends.
//!
//! ## API Modules
//!
//! - [`motion`] — one-shot motion and rotation animations
//! - [`tween`] — looping tween animations
//! - [`fx`] — visual effects such as fades, scaling, and tints
//! - [`layer`] — runtime sprite/variant swapping
//! - [`anim`] — character behavior animations (blinking, talking)
//! - [`callback`] — animation lifecycle callbacks
//!
//! ## Usage
//!
//! Frontends can import the entire API surface through this module:
//!
//! ```ignore
//! use emotiva::api::*;
//! ```
//!
//! This provides access to the [`EmotivaAPI`] trait and all related
//! animation control methods.
//!
//! ## Design Goal
//!
//! The API layer keeps Emotiva's runtime **clean and modular** by separating
//! the public animation interface from the internal animation systems
//! implemented in the `core` modules.

pub mod anim;
pub mod callback;
pub mod fx;
pub mod layer;
pub mod motion;
pub mod tween;

pub use crate::core::easing::Easing;

pub trait EmotivaAPI {
    // Eyes
    fn eyes_is_blinking(&self) -> bool;
    fn eyes_is_blinking_enabled(&self) -> bool;
    fn eyes_start(&mut self) -> Option<u64>;
    fn eyes_stop(&mut self);
    fn eyes_set_blink_duration(&mut self, duration: f32);
    fn eyes_set_blink_interval_range(&mut self, range: (f32, f32));
    // Mouth
    fn mouth_is_talking(&self) -> bool;
    fn mouth_is_talking_enabled(&self) -> bool;
    fn mouth_start(&mut self) -> Option<u64>;
    fn mouth_stop(&mut self);
    fn mouth_set_talk_duration(&mut self, duration: f32);
    fn mouth_set_talk_interval(&mut self, interval: f32);
    fn mouth_set_flap_open_time(&mut self, duration: f32);
    // Layer
    fn set_layer(&mut self, layer_name: &str, variant: &str);
    fn reset_layer(&mut self, layer_name: &str);
    // Motion
    fn motion_play(&mut self, layer: &str) -> Option<u64>;
    fn motion_reverse(&mut self, layer: &str) -> Option<u64>;
    fn rotation_play(&mut self, layer: &str) -> Option<u64>;
    fn rotation_reverse(&mut self, layer: &str) -> Option<u64>;
    fn is_motion_playing(&self, layer: &str) -> bool;
    fn is_rotation_playing(&self, layer: &str) -> bool;
    // Tween
    fn tween_start(&mut self, layer: &str) -> Option<u64>;
    fn tween_stop(&mut self, layer: &str);
    fn tween_pause(&mut self, layer: &str);
    fn tween_resume(&mut self, layer: &str);
    fn is_tween_enabled(&self, layer: &str) -> bool;
    fn is_tween_paused(&self, layer: &str) -> bool;
    // FX
    fn set_scale(&mut self, layer: &str, from: f32, to: f32, duration: f32, easing: Easing) -> u64;
    fn remove_scale(&mut self, layer: &str);
    fn set_alpha(&mut self, layer: &str, from: f32, to: f32, duration: f32, easing: Easing) -> u64;
    fn remove_alpha(&mut self, layer: &str);
    fn set_tint(
        &mut self,
        layer: &str,
        from: [f32; 4],
        to: [f32; 4],
        duration: f32,
        easing: Easing,
    ) -> u64;
    fn remove_tint(&mut self, layer: &str);
    fn clear_all_fx(&mut self);
    fn is_scale_finished(&self, layer: &str) -> bool;
    fn is_alpha_finished(&self, layer: &str) -> bool;
    fn is_tint_finished(&self, layer: &str) -> bool;
    // Delay
    fn set_delay(&mut self, duration: f32) -> u64;
}
