//! delay.rs – Minimal delay animation module for Emotiva
//!
//! This module provides a tiny, self-contained "delay animation" type. It doesn’t change any
//! sprite properties – it simply waits for a specified duration, then fires an `AnimEvent::Completed`
//! so that Emotiva’s callback system can chain the next action.
//!
//! This keeps Emotiva’s core tiny and composable: delays become just another animation type,
//! allowing developers to sequence animations without a timeline engine or queue logic.

use crate::AnimEvent;

#[derive(Debug, Clone)]
pub struct Delay {
    /// Total time the delay should last (in seconds)
    pub duration: f32,
    /// Elapsed time since this delay started
    pub elapsed: f32,
    /// Whether this delay has been triggered once (used for firing `Started` event)
    pub started: bool,
    /// Whether this delay is finished
    pub finished: bool,
    /// Optional animation ID (used for callbacks)
    pub animation_id: Option<u64>,
}

impl Delay {
    /// Create a new delay animation with the given duration in seconds.
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            elapsed: 0.0,
            started: false,
            finished: false,
            animation_id: None,
        }
    }

    /// Attach an animation ID for callback tracking.
    pub fn set_animation_id(&mut self, id: u64) {
        self.animation_id = Some(id);
    }

    /// Reset this delay to its starting state.
    pub fn play(&mut self) {
        self.elapsed = 0.0;
        self.started = false;
        self.finished = false;
    }

    /// Update this delay by delta time.
    ///
    /// Returns:
    /// - `AnimEvent::Started` once when the delay begins
    /// - `AnimEvent::Completed` once when the delay finishes
    /// - `AnimEvent::None` while the delay is still counting down
    pub fn update(&mut self, dt: f32) -> AnimEvent {
        if self.finished {
            return AnimEvent::None;
        }

        self.elapsed += dt;

        // Fire Started event the first time we tick
        if !self.started {
            self.started = true;
            if self.elapsed >= self.duration {
                self.finished = true;
                return AnimEvent::Completed(self.animation_id);
            }
            return AnimEvent::Started(self.animation_id);
        }

        // Continue ticking until duration is reached
        if self.elapsed >= self.duration {
            self.finished = true;
            AnimEvent::Completed(self.animation_id)
        } else {
            AnimEvent::None
        }
    }

    /// Check if this delay has completed.
    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
