//! # 🎭 Tween API
//!
//! Public API for controlling **tween-based animations** in Emotiva.
//!
//! This module extends [`EmotivaHeart`] with helper methods for managing
//! looping tween animations attached to character layers.
//!
//! Tweens are commonly used for subtle continuous movements such as
//! breathing, idle motion, or gentle sprite offsets that bring a
//! character to life.
//!
//! ## Responsibilities
//!
//! This module provides APIs to:
//!
//! - Start tween animations
//! - Stop tween animations
//! - Pause and resume tweens
//! - Query tween state (enabled or paused)
//!
//! ## Example
//!
//! ```ignore
//! heart.tween_start("body");
//!
//! if heart.is_tween_enabled("body") {
//!     heart.tween_pause("body");
//! }
//!
//! heart.tween_resume("body");
//! ```
//!
//! These methods act as a thin **control layer** over Emotiva's internal
//! tween system, allowing frontends to trigger continuous character
//! movements without directly interacting with low-level tween state.

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
