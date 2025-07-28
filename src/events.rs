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
//! EmotivaCore consumes these events inside its `update()` loop to trigger
//! callbacks, chain animations, and keep different modules synchronized without
//! unsafe code or ad-hoc flags.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimEvent {
    None,
    Started,
    Completed,
    Paused,
    Resumed,
    Reversed,
}
