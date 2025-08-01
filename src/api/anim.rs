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
            ("eyes", "start_blinking") => {
                if let Some(eyes) = &mut self.eyes {
                    eyes.start();
                }
            }
            ("eyes", "stop_blinking") => {
                if let Some(eyes) = &mut self.eyes {
                    eyes.stop();
                }
            }
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
