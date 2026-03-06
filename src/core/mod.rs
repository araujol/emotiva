//! # 💠 Emotiva Core
//!
//! Internal engine components powering Emotiva's animation systems.
//!
//! This module contains the backend-agnostic building blocks used to
//! implement Emotiva's runtime behavior, including animation engines,
//! easing functions, visual effects, and timing utilities.
//!
//! These modules are primarily **internal implementation details** and
//! are not intended to be used directly by frontend code. Most users
//! should interact with Emotiva through the [`crate::api`] module
//! instead.
//!
//! ## Core Modules
//!
//! - [`delay`] — timing utilities for scheduling animation actions
//! - [`easing`] — easing functions for smooth interpolation
//! - [`events`] — animation event definitions and dispatching
//! - [`fx`] — visual effects systems (alpha, tint, scale, etc.)
//! - [`motion`] — motion and positional animation logic
//! - [`palette`] — color palette helpers
//! - [`transform`] — transformation utilities (scale, rotation, position)
//! - [`tween`] — tweening engine for value interpolation
//!
//! ## Design Goal
//!
//! The `core` module isolates the **low-level animation systems**
//! that power Emotiva. Higher-level runtime controls are provided
//! through the [`crate::api`] layer.

pub mod delay;
pub mod easing;
pub mod events;
pub mod fx;
pub mod motion;
pub mod palette;
pub mod transform;
pub mod tween;
