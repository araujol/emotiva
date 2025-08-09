//! Emotiva Event System
//!
//! This module defines `AnimEvent`, the shared signaling enum used by all
//! animation subsystems in EmotivaCore (tween, motion, FX).
//!
//! Each time an animation changes state, it returns one of these variants:
//! - `None`      → No significant state change this frame
//! - `Started`   → Animation has just begun
//! - `Completed` → Animation has finished playing
//! - `Paused`    → Animation was paused mid-progress
//! - `Resumed`   → A previously paused animation resumed
//! - `Reversed`  → Animation direction flipped (e.g. forward → reverse)
//!
//! Each event can optionally include an **animation ID** to uniquely identify
//! which animation instance triggered the event. This prevents callbacks from
//! mixing up overlapping animations on the same layer.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimEvent {
    /// No state change this frame (no animation ID needed)
    None,
    /// Animation has just begun
    Started(Option<u64>),
    /// Animation has finished playing
    Completed(Option<u64>),
    /// Animation was paused mid-progress
    Paused(Option<u64>),
    /// A previously paused animation resumed
    Resumed(Option<u64>),
    /// Animation direction flipped (e.g. forward → reverse)
    Reversed(Option<u64>),
}

impl AnimEvent {
    /// Convenience constructor for events without IDs.
    pub fn no_id(self) -> Self {
        match self {
            AnimEvent::Started(_) => AnimEvent::Started(None),
            AnimEvent::Completed(_) => AnimEvent::Completed(None),
            AnimEvent::Paused(_) => AnimEvent::Paused(None),
            AnimEvent::Resumed(_) => AnimEvent::Resumed(None),
            AnimEvent::Reversed(_) => AnimEvent::Reversed(None),
            AnimEvent::None => AnimEvent::None,
        }
    }

    /// Check if this event has an animation ID.
    pub fn id(&self) -> Option<u64> {
        match self {
            AnimEvent::Started(id)
            | AnimEvent::Completed(id)
            | AnimEvent::Paused(id)
            | AnimEvent::Resumed(id)
            | AnimEvent::Reversed(id) => *id,
            AnimEvent::None => None,
        }
    }
}
