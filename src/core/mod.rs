// ==========================================
// 💠 Emotiva Core Module (core/mod.rs)
// ------------------------------------------
// This module defines the **internal engine components** of Emotiva.
//
// ✅ Responsibilities:
//  - Provide the backend-agnostic building blocks for animation and FX.
//  - Contain reusable logic shared across all frontends.
//  - Keep implementation details isolated from public-facing API.
//
// 📦 Structure:
//  - delay.rs     → timing utilities for scheduling actions.
//  - easing.rs    → easing functions for smooth animations.
//  - events.rs    → animation event definitions & handling.
//  - fx.rs        → visual effects (alpha, tint, etc.).
//  - motion.rs    → movement and position animations.
//  - palette.rs   → color palette definitions.
//  - transform.rs → transformation utilities (scale, rotation, position).
//  - tween.rs     → tweening engine for value interpolation.
//
// These modules are **not** meant to be accessed directly by end-users —
// they are exposed selectively through the `api` module.
// ==========================================

pub mod delay;
pub mod easing;
pub mod events;
pub mod fx;
pub mod motion;
pub mod palette;
pub mod transform;
pub mod tween;
