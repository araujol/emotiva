// ==========================================
// 🎭 Emotiva Tween API Module
// ------------------------------------------
// This module defines the public-facing API
// functions for controlling tweens in Emotiva.
//
// ✅ Responsibilities:
//  - Start, stop, pause, and resume tweens
//  - Handle easing-based tween operations
//  - Query tween state (enabled, paused)
//
// 📦 Usage:
// These functions are methods on EmotivaHeart and
// provide a clean way for frontends to manipulate
// character part animations (e.g. breathing, idle)
// without directly touching internal tween structs.
// ==========================================

use crate::EmotivaHeart;

impl EmotivaHeart {
    /// Starts a looping **tween** on the specified layer.
    ///
    /// * `layer` - The name of the layer to tween.
    ///
    /// Returns an animation ID as Some(ID) for tracking if it suceeds, or None
    /// if it failed.
    pub fn tween_start(&mut self, layer: &str) -> Option<u64> {
        if self.tweens.contains_key(layer) {
            let id = self.next_id();
            if let Some(tween) = self.tweens.get_mut(layer) {
                tween.set_animation_id(id);
                if let Some(tween_def) = self
                    .rig
                    .layers
                    .iter()
                    .find(|l| l.name == layer)
                    .and_then(|l| l.tween.as_ref())
                {
                    tween.start_with_tween(tween_def);
                } else {
                    tween.start();
                }
                return Some(id);
            }
        }
        None
    }

    /// Stops any running **tween** on the specified layer immediately.
    pub fn tween_stop(&mut self, layer: &str) {
        if let Some(tween) = self.tweens.get_mut(layer) {
            if let Some(tween_def) = self
                .rig
                .layers
                .iter()
                .find(|l| l.name == layer)
                .and_then(|l| l.tween.as_ref())
            {
                tween.stop_with_tween(tween_def);
            } else {
                tween.stop();
            }
        }
    }

    /// Pauses the tween on the specified layer (keeps current position frozen).
    pub fn tween_pause(&mut self, layer: &str) {
        if let Some(tween) = self.tweens.get_mut(layer) {
            tween.pause();
        }
    }

    /// Resumes a paused tween on the specified layer.
    pub fn tween_resume(&mut self, layer: &str) {
        if let Some(tween) = self.tweens.get_mut(layer) {
            tween.resume();
        }
    }

    /// Checks if the tween on the specified layer is currently **enabled**.
    ///
    /// Returns `true` if the tween is active, otherwise `false`.
    pub fn is_tween_enabled(&self, layer: &str) -> bool {
        if let Some(tween) = self.tweens.get(layer) {
            return tween.is_enabled();
        }
        false
    }

    /// Checks if the tween on the specified layer is currently **paused**.
    ///
    /// Returns `true` if paused, otherwise `false`.
    pub fn is_tween_paused(&self, layer: &str) -> bool {
        if let Some(tween) = self.tweens.get(layer) {
            return tween.is_paused();
        }
        false
    }
}
