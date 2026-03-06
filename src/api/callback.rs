//! # 🎭 Callback API
//!
//! Public API for attaching **callbacks to animation events** in Emotiva.
//!
//! This module extends [`EmotivaHeart`] with helper methods that allow
//! developers to execute custom logic when an animation starts or finishes.
//!
//! Callbacks are associated with animation IDs returned by animation
//! functions (such as FX, motions, or tweens), allowing frontends to
//! react to animation lifecycle events.
//!
//! ## Responsibilities
//!
//! This module provides APIs to:
//!
//! - Register callbacks when an animation **starts**
//! - Register callbacks when an animation **completes**
//! - Execute user-defined closures when those events occur
//!
//! ## Example
//!
//! ```ignore
//! let id = heart.set_alpha("body", 0.0, 1.0, 0.5, Easing::EaseOut);
//!
//! heart.on_end(id, |emo| {
//!     println!("Fade finished!");
//! });
//! ```
//!
//! Callbacks enable simple animation chaining and event-driven behavior
//! without requiring frontends to constantly poll animation state.

use crate::EmotivaHeart;

impl EmotivaHeart {
    pub fn on_start<F>(&mut self, id: u64, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        self.register_callback_on_start(id, cb);
    }

    pub fn on_end<F>(&mut self, id: u64, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        self.register_callback_on_complete(id, cb);
    }
}
