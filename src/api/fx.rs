// ==========================================
// 🎭 Emotiva FX API Module
// ------------------------------------------
// This module defines the public-facing API
// functions for managing **visual effects** in Emotiva.
//
// ✅ Responsibilities:
//  - Add or remove scale, alpha (fade), and tint FX
//  - Clear all FX in one call
//  - Wrap lower-level FX structs with friendly API methods
//
// 📦 Usage:
// These methods provide an easy way for frontends to
// trigger visual changes (fades, size changes, color tints)
// on layers. They allow chaining or timing with tweens/motions
// for expressive animated effects.
// ==========================================

use crate::EmotivaHeart;
use crate::core::easing::Easing;
use crate::core::fx::{make_alpha_fx, make_scale_fx, make_tint_fx};

impl EmotivaHeart {
    /// Starts a scaling FX on the specified layer.
    ///
    /// * `layer` - The target layer name.
    /// * `from` - Initial scale value.
    /// * `to` - Target scale value.
    /// * `duration` - Duration in seconds.
    /// * `easing` - Easing function for the scale transition.
    ///
    /// Returns a unique FX ID for tracking.
    pub fn set_scale(
        &mut self,
        layer: &str,
        from: f32,
        to: f32,
        duration: f32,
        easing: Easing,
    ) -> u64 {
        let id = self.next_id();
        self.visual_fx
            .add_scale_fx(layer, make_scale_fx(from, to, duration, easing, Some(id)));
        id
    }

    /// Removes any active scaling FX from the specified layer.
    pub fn remove_scale(&mut self, layer: &str) {
        self.visual_fx.remove_scale_fx(layer);
    }

    /// Starts an alpha FX (fade) on the specified layer.
    ///
    /// * `layer` - The target layer name.
    /// * `from` - Initial alpha value (0.0 = transparent, 1.0 = fully opaque).
    /// * `to` - Target alpha value.
    /// * `duration` - Duration in seconds.
    /// * `easing` - Easing function for the fade.
    ///
    /// Returns a unique FX ID for tracking.
    pub fn set_alpha(
        &mut self,
        layer: &str,
        from: f32,
        to: f32,
        duration: f32,
        easing: Easing,
    ) -> u64 {
        let id = self.next_id();
        self.visual_fx
            .add_alpha_fx(layer, make_alpha_fx(from, to, duration, easing, Some(id)));
        id
    }

    /// Removes any active alpha FX (fade) from the specified layer.
    pub fn remove_alpha(&mut self, layer: &str) {
        self.visual_fx.remove_alpha_fx(layer);
    }

    /// Starts a tint FX on the specified layer.
    ///
    /// * `layer` - The target layer name.
    /// * `from` - Initial RGBA color.
    /// * `to` - Target RGBA color.
    /// * `duration` - Duration in seconds.
    /// * `easing` - Easing function for the tint transition.
    ///
    /// Returns a unique FX ID for tracking.
    pub fn set_tint(
        &mut self,
        layer: &str,
        from: [f32; 4],
        to: [f32; 4],
        duration: f32,
        easing: Easing,
    ) -> u64 {
        let id = self.next_id();
        self.visual_fx
            .add_tint_fx(layer, make_tint_fx(from, to, duration, easing, Some(id)));
        id
    }

    /// Removes any active tint FX from the specified layer.
    pub fn remove_tint(&mut self, layer: &str) {
        self.visual_fx.remove_tint_fx(layer);
    }

    /// Clears **all** active FX (scale, alpha, tint) from all layers.
    pub fn clear_all_fx(&mut self) {
        self.visual_fx.clear_all_fx();
    }
}
