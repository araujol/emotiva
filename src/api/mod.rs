// src/api/mod.rs
// ==========================================
// 🎭 Emotiva API Module (mod.rs)
// ------------------------------------------
// This is the **main API entry point** for Emotiva.
//
// ✅ Responsibilities:
//  - Organize and re-export all sub-API modules (tween, motion, fx, anim)
//  - Serve as a single import point for all public-facing Emotiva functions
//  - Provide a clear structure to keep lib.rs clean and maintainable
//
// 📦 Usage:
// Frontend projects only need to import from this module to access the
// full Emotiva API surface:
// ```rust
// use emotiva::api::*;
// ```
// ==========================================

pub mod anim;
pub mod callback;
pub mod fx;
pub mod layer;
pub mod motion;
pub mod tween;

use crate::easing::Easing;

pub trait EmotivaAPI {
    fn trigger(&mut self, layer: &str, action: &str);
    // Layer
    fn set_layer(&mut self, layer_name: &str, variant: &str);
    fn reset_layer(&mut self, layer_name: &str);
    // Motion
    fn motion_play(&mut self, layer: &str) -> u64;
    fn motion_reverse(&mut self, layer: &str) -> u64;
    fn rotation_play(&mut self, layer: &str) -> u64;
    fn rotation_reverse(&mut self, layer: &str) -> u64;
    fn is_motion_finished(&self, layer: &str) -> bool;
    fn is_rotation_finished(&self, layer: &str) -> bool;
    // Tween
    fn tween_start(&mut self, layer: &str) -> u64;
    fn tween_stop(&mut self, layer: &str);
    fn tween_start_easing(&mut self, layer: &str) -> u64;
    fn tween_stop_easing(&mut self, layer: &str);
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
    // Delay
    fn set_delay(&mut self, duration: f32) -> u64;
}
