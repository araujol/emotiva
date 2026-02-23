// ==========================================
// 🎭 Emotiva Motion API Module
// ------------------------------------------
// This module defines the public-facing API
// functions for triggering one-shot motions
// and rotations in Emotiva.
//
// ✅ Responsibilities:
//  - Play and reverse motion animations
//  - Play and reverse rotation animations
//  - Query whether motion or rotation has finished
//
// 📦 Usage:
// These API methods are attached to EmotivaHeart,
// allowing frontends to trigger one-time moves or
// rotations (e.g. a head tilt or nod) without
// dealing with lower-level state management.
// ==========================================

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
        let motion_done = self
            .motions
            .get(layer)
            .map(|m| m.is_playing())
            .unwrap_or(true);
        motion_done
    }

    /// Checks if the **rotation** animation on the given layer is playing.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns `true` if the rotation is playing.
    pub fn is_rotation_playing(&self, layer: &str) -> bool {
        let rotation_done = self
            .rotations
            .get(layer)
            .map(|r| r.is_playing())
            .unwrap_or(true);
        rotation_done
    }
}
