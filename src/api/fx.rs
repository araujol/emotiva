//! # 🎭 FX API
//!
//! Public API for controlling **visual effects (FX)** in Emotiva.
//!
//! This module extends [`EmotivaHeart`] with helper methods that allow
//! frontends to apply temporary visual transformations to character
//! layers.
//!
//! These effects are commonly used to create expressive visual changes
//! such as fades, scale pulses, or color tints that can be combined
//! with motions and tweens.
//!
//! ## Responsibilities
//!
//! This module provides APIs to:
//!
//! - Apply scale FX to layers
//! - Apply alpha FX (fades)
//! - Apply tint FX (color transitions)
//! - Remove FX from individual layers
//! - Clear all active FX at once
//! - Query whether an FX animation has finished
//!
//! ## Example
//!
//! ```ignore
//! heart.set_alpha("body", 0.0, 1.0, 0.5, Easing::EaseOut);
//!
//! if heart.is_alpha_finished("body") {
//!     heart.set_tint("body", [1.0,1.0,1.0,1.0], [1.0,0.8,0.8,1.0], 0.3, Easing::EaseIn);
//! }
//! ```
//!
//! These methods act as a thin **runtime control layer** over Emotiva's
//! internal FX system, making it easy for frontends to trigger visual
//! effects without interacting with lower-level effect structures.

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

    /// Returns true if the scale FX on the specified layer has finished, or if none exists.
    pub fn is_scale_finished(&self, layer: &str) -> bool {
        self.visual_fx.is_scale_finished(layer)
    }

    /// Returns true if the alpha FX on the specified layer has finished, or if none exists.
    pub fn is_alpha_finished(&self, layer: &str) -> bool {
        self.visual_fx.is_alpha_finished(layer)
    }

    /// Returns true if the tint FX on the specified layer has finished, or if none exists.
    pub fn is_tint_finished(&self, layer: &str) -> bool {
        self.visual_fx.is_tint_finished(layer)
    }
}
