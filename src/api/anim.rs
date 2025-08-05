// ==========================================
// 🎭 Emotiva Anim API Module
// ------------------------------------------
// This module defines API functions for triggering
// **character behavior animations** in Emotiva.
//
// ✅ Responsibilities:
//  - Provide a simple trigger-based interface for
//    common animations (blinking, talking, idle chat)
//  - Dispatch actions to eyes and mouth animation subsystems
//  - Handle unknown triggers gracefully with debug output
//
// 📦 Usage:
// These methods allow frontends to issue simple text-based
// animation commands (e.g. "start_blinking", "stop_talking")
// without manually controlling lower-level animation state.
// ==========================================

use crate::EmotivaHeart;

impl EmotivaHeart {
    // ================= Eyes API =================

    /// Starts the automatic blinking animation loop.
    ///
    /// This enables periodic eye blinking for the active character.
    ///
    /// Returns a unique eyes ID for tracking.
    pub fn eyes_start(&mut self) -> u64 {
        let id = self.assign_id_to_eyes();
        if let Some(eyes) = &mut self.eyes {
            eyes.start();
        }
        id
    }

    /// Stops the automatic blinking animation loop.
    ///
    /// This disables blinking entirely and keeps the eyes open.
    pub fn eyes_stop(&mut self) {
        if let Some(eyes) = &mut self.eyes {
            eyes.stop();
        }
    }

    /// Returns `true` if the eyes are currently in the closed-blinking state.
    ///
    /// Useful for synchronizing expressions or visual effects with active blinks.
    pub fn eyes_is_blinking(&self) -> bool {
        self.eyes
            .as_ref()
            .map(|eyes| eyes.is_blinking())
            .unwrap_or(false)
    }

    /// Returns `true` if blinking is currently enabled (i.e. a session is active).
    ///
    /// This can be used to determine if a blink session is in progress,
    /// regardless of whether the eyes are currently open or closed.
    pub fn eyes_is_blinking_active(&self) -> bool {
        self.eyes
            .as_ref()
            .map(|eyes| eyes.is_blinking_active())
            .unwrap_or(false)
    }

    /// Sets the blinking interval range for the eyes.
    ///
    /// * `range` - A tuple of `(min, max)` in seconds specifying how often blinks occur.
    ///             For example, `(2.0, 5.0)` causes a blink every 2–5 seconds.
    pub fn eyes_set_blink_interval(&mut self, range: (f32, f32)) {
        if let Some(eyes) = &mut self.eyes {
            eyes.set_interval_range(range);
        }
    }

    /// Sets the blink duration for the eyes.
    ///
    /// * `duration` - The number of seconds the eyes stay closed per blink.
    ///                For example, `0.12` simulates a quick blink.
    pub fn eyes_set_blink_duration(&mut self, duration: f32) {
        if let Some(eyes) = &mut self.eyes {
            eyes.set_blink_duration(duration);
        }
    }

    /// Triggers a predefined **animation action** on a specified layer.
    ///
    /// # Parameters
    /// - `layer`: The target layer name (e.g. "eyes" or "mouth").
    /// - `action`: The action to perform (e.g. "start_blinking", "stop_talking", "idle_chat").
    ///
    /// # Behavior
    /// - If the layer and action are known, the corresponding animation method is called.
    /// - If the layer/action combination is unknown, an error message is printed to stderr.
    pub fn trigger(&mut self, layer: &str, action: &str) {
        match (layer, action) {
            ("mouth", "start_talking") => {
                if let Some(mouth) = &mut self.mouth {
                    mouth.start();
                }
            }
            ("mouth", "stop_talking") => {
                if let Some(mouth) = &mut self.mouth {
                    mouth.stop();
                }
            }
            ("mouth", "idle_chat") => {
                if let Some(mouth) = &mut self.mouth {
                    mouth.idle_chat();
                }
            }
            _ => {
                eprintln!("Unknown trigger: {}/{}", layer, action);
            }
        }
    }
}
