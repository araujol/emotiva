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
// These functions are methods on EmotivaCore and
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
    /// Returns a unique tween ID for tracking.
    pub fn tween_start(&mut self, layer: &str) -> u64 {
        let id = self.assign_id_to_tween(layer);
        if let Some(tween) = self.tweens.get_mut(layer) {
            tween.start()
        }
        id
    }

    /// Stops any running **tween** on the specified layer immediately.
    pub fn tween_stop(&mut self, layer: &str) {
        if let Some(tween) = self.tweens.get_mut(layer) {
            tween.stop();
        }
    }

    /// Starts a **tween** with easing values defined in the `.ron` rig file.
    ///
    /// * `layer` - The name of the layer to tween.
    ///
    /// Returns a unique tween ID for tracking.
    pub fn tween_start_easing(&mut self, layer: &str) -> u64 {
        let id = self.assign_id_to_tween(layer);
        if let Some(tween) = self.tweens.get_mut(layer) {
            if let Some(layer_def) = self.rig.layers.iter().find(|l| l.name == layer) {
                if let Some(tween_def) = &layer_def.tween {
                    tween.start_easing(tween_def);
                }
            }
        }
        id
    }

    /// Stops a **tween** with easing values defined in the `.ron` rig file.
    pub fn tween_stop_easing(&mut self, layer: &str) {
        if let Some(tween) = self.tweens.get_mut(layer) {
            if let Some(layer_def) = self.rig.layers.iter().find(|l| l.name == layer) {
                if let Some(tween_def) = &layer_def.tween {
                    tween.stop_easing(tween_def);
                }
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
    pub fn is_tween_enabled(&mut self, layer: &str) -> bool {
        if let Some(tween) = self.tweens.get_mut(layer) {
            return tween.is_enabled();
        }
        false
    }

    /// Checks if the tween on the specified layer is currently **paused**.
    ///
    /// Returns `true` if paused, otherwise `false`.
    pub fn is_tween_paused(&mut self, layer: &str) -> bool {
        if let Some(tween) = self.tweens.get_mut(layer) {
            return tween.is_paused();
        }
        false
    }
}
