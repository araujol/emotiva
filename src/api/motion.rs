//! # 🎭 Motion API
//!
//! Public API for triggering **one-shot motion and rotation animations**
//! in Emotiva.
//!
//! This module extends [`EmotivaHeart`] with helper methods that allow
//! frontends to trigger short animation actions without interacting with
//! the lower-level animation systems directly.
//!
//! These animations are typically used for small expressive movements,
//! such as a head tilt, nod, or brief body motion.
//!
//! ## Responsibilities
//!
//! This module provides APIs to:
//!
//! - Play motion animations
//! - Reverse motion animations
//! - Play rotation animations
//! - Reverse rotation animations
//! - Query whether a motion or rotation animation is currently playing
//!
//! ## Example
//!
//! ```ignore
//! heart.motion_play("head");
//!
//! if !heart.is_motion_playing("head") {
//!     heart.rotation_play("body");
//! }
//! ```
//!
//! The module acts as a thin **control layer** over Emotiva's internal
//! motion and rotation animation systems.

use crate::EmotivaHeart;

impl EmotivaHeart {
    /// Plays a forward one-shot **motion** animation on the given layer.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns an animation ID as Some(ID) for tracking if it suceeds, or None
    /// if it failed.
    pub fn motion_play(&mut self, layer: &str) -> Option<u64> {
        if self.motions.contains_key(layer) {
            let id = self.next_id();
            if let Some(motion) = self.motions.get_mut(layer) {
                motion.set_animation_id(id);
                motion.play();
                return Some(id);
            }
        }
        None
    }

    /// Plays the **reverse** of the motion animation on the given layer.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns an animation ID as Some(ID) for tracking if it suceeds, or None
    /// if it failed.
    pub fn motion_reverse(&mut self, layer: &str) -> Option<u64> {
        if self.motions.contains_key(layer) {
            let id = self.next_id();
            if let Some(motion) = self.motions.get_mut(layer) {
                motion.set_animation_id(id);
                motion.reverse();
                return Some(id);
            }
        }
        None
    }

    /// Plays a forward one-shot **rotation** animation on the given layer.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns an animation ID as Some(ID) for tracking if it suceeds, or None
    /// if it failed.
    pub fn rotation_play(&mut self, layer: &str) -> Option<u64> {
        if self.rotations.contains_key(layer) {
            let id = self.next_id();
            if let Some(rotation) = self.rotations.get_mut(layer) {
                rotation.set_animation_id(id);
                rotation.play();
                return Some(id);
            }
        }
        None
    }

    /// Plays the **reverse** of the rotation animation on the given layer.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns an animation ID as Some(ID) for tracking if it suceeds, or None
    /// if it failed.
    pub fn rotation_reverse(&mut self, layer: &str) -> Option<u64> {
        if self.rotations.contains_key(layer) {
            let id = self.next_id();
            if let Some(rotation) = self.rotations.get_mut(layer) {
                rotation.set_animation_id(id);
                rotation.reverse();
                return Some(id);
            }
        }
        None
    }

    /// Checks if the **motion** animation on the given layer is playing.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns `true` if the motion is playing.
    pub fn is_motion_playing(&self, layer: &str) -> bool {
        self.motions
            .get(layer)
            .map(|m| m.is_playing())
            .unwrap_or(false)
    }

    /// Checks if the **rotation** animation on the given layer is playing.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns `true` if the rotation is playing.
    pub fn is_rotation_playing(&self, layer: &str) -> bool {
        self.rotations
            .get(layer)
            .map(|r| r.is_playing())
            .unwrap_or(false)
    }
}
