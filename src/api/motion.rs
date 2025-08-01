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
// These API methods are attached to EmotivaCore,
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
    /// Returns a unique motion ID for tracking.
    pub fn motion_play(&mut self, layer: &str) -> u64 {
        let id = self.assign_id_to_motion(layer);
        if let Some(motion) = self.motions.get_mut(layer) {
            motion.play()
        }
        id
    }

    /// Plays the **reverse** of the motion animation on the given layer.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns a unique motion ID for tracking.
    pub fn motion_reverse(&mut self, layer: &str) -> u64 {
        let id = self.assign_id_to_motion(layer);
        if let Some(motion) = self.motions.get_mut(layer) {
            motion.reverse()
        }
        id
    }

    /// Plays a forward one-shot **rotation** animation on the given layer.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns a unique rotation ID for tracking.
    pub fn rotation_play(&mut self, layer: &str) -> u64 {
        let id = self.assign_id_to_rotation(layer);
        if let Some(rotation) = self.rotations.get_mut(layer) {
            rotation.play()
        }
        id
    }

    /// Plays the **reverse** of the rotation animation on the given layer.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns a unique rotation ID for tracking.
    pub fn rotation_reverse(&mut self, layer: &str) -> u64 {
        let id = self.assign_id_to_rotation(layer);
        if let Some(rotation) = self.rotations.get_mut(layer) {
            rotation.reverse()
        }
        id
    }

    /// Checks if the **motion** animation on the given layer has finished.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns `true` if the motion is done or the layer has no motion.
    pub fn is_motion_finished(&self, layer: &str) -> bool {
        let motion_done = self
            .motions
            .get(layer)
            .map(|m| m.is_finished())
            .unwrap_or(true);
        motion_done
    }

    /// Checks if the **rotation** animation on the given layer has finished.
    ///
    /// * `layer` - Name of the target layer.
    ///
    /// Returns `true` if the rotation is done or the layer has no rotation.
    pub fn is_rotation_finished(&self, layer: &str) -> bool {
        let rotation_done = self
            .rotations
            .get(layer)
            .map(|r| r.is_finished())
            .unwrap_or(true);
        rotation_done
    }
}
