// ==========================================
// 🎭 Emotiva Anim API Module
// ------------------------------------------
// This module defines API functions for triggering
// **character behavior animations** in Emotiva.
//
// ✅ Responsibilities:
//  - Provide a simple API for common animations (blinking, talking)
//  - Dispatch actions to eyes and mouth animation subsystems
//
// 📦 Usage:
// These methods allow frontends to access the animation API.
//
// ✨ Currently implemented:
//  - Eyes API: blinking animations with duration and interval
//  - Mouth API: talking animations with configurable flap rhythm
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
    pub fn eyes_is_blinking_enabled(&self) -> bool {
        self.eyes
            .as_ref()
            .map(|eyes| eyes.is_blinking_enabled())
            .unwrap_or(false)
    }

    /// Sets the blinking interval range for the eyes.
    ///
    /// * `range` - A tuple of `(min, max)` in seconds specifying how often blinks occur.
    ///             For example, `(2.0, 5.0)` causes a blink every 2–5 seconds.
    pub fn eyes_set_blink_interval_range(&mut self, range: (f32, f32)) {
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

    // ================= Mouth API =================

    /// Starts the automatic talking animation loop.
    ///
    /// This enables repeated mouth flaps for the configured talk duration.
    /// Returns a unique mouth ID for tracking.
    pub fn mouth_start(&mut self) -> u64 {
        let id = self.assign_id_to_mouth();
        if let Some(mouth) = &mut self.mouth {
            mouth.start();
        }
        id
    }

    /// Stops the talking animation loop.
    ///
    /// This immediately disables mouth flaps and resets the mouth to idle state.
    pub fn mouth_stop(&mut self) {
        if let Some(mouth) = &mut self.mouth {
            mouth.stop();
        }
    }

    /// Returns `true` if the mouth is currently flapping open.
    ///
    /// This can be used to detect if a flap is actively in progress.
    pub fn mouth_is_talking(&self) -> bool {
        self.mouth
            .as_ref()
            .map(|mouth| mouth.is_talking())
            .unwrap_or(false)
    }

    /// Returns `true` if a talk session is currently active.
    ///
    /// This indicates that the mouth is in a state where flaps are allowed to trigger.
    pub fn mouth_is_talking_enabled(&self) -> bool {
        self.mouth
            .as_ref()
            .map(|mouth| mouth.is_talking_enabled())
            .unwrap_or(false)
    }

    /// Sets how often talk sessions can occur.
    ///
    /// * `interval` - Delay in seconds between talk sessions.
    pub fn mouth_set_talk_interval(&mut self, interval: f32) {
        if let Some(mouth) = &mut self.mouth {
            mouth.set_talk_interval(interval);
        }
    }

    /// Sets how long each talk session lasts.
    ///
    /// * `duration` - Total time in seconds that mouth flaps continue once started.
    pub fn mouth_set_talk_duration(&mut self, duration: f32) {
        if let Some(mouth) = &mut self.mouth {
            mouth.set_talk_duration(duration);
        }
    }

    /// Sets how long the mouth stays open during each flap.
    ///
    /// * `duration` - Seconds the mouth remains open before closing again.
    pub fn mouth_set_flap_open_time(&mut self, duration: f32) {
        if let Some(mouth) = &mut self.mouth {
            mouth.set_flap_open_time(duration);
        }
    }
}
