//! # 📦 Emotiva Rig Format
//!
//! Serializable data structures used to define **character rigs** in Emotiva.
//!
//! This module describes the schema for `.ron` rig files that specify how a
//! character is built and animated. These structures are deserialized into an
//! [`EmotivaRig`] and consumed by the runtime animation engine.
//!
//! The rig format acts as the bridge between **static character definitions**
//! and the **runtime animation systems** implemented in the engine.
//!
//! ## Core Concepts
//!
//! An Emotiva rig is composed of **ordered image layers** that may include
//! optional animation definitions.
//!
//! Each layer can define:
//!
//! - A base image
//! - Position offsets and scale
//! - Drawing order (`z_index`)
//! - Optional tween animations
//! - Optional motion animations
//! - Optional image variants for runtime swapping
//!
//! Additional configuration blocks may define **character behaviors** such as
//! blinking or talking.
//!
//! ## Main Structures
//!
//! - [`EmotivaRig`] — top-level character rig definition
//! - [`Layer`] — individual sprite layer definition
//! - [`Tween`] — looping sway/lean animation definition
//! - [`MotionDef`] — one-shot motion animation
//! - [`EyesConfig`] — blinking behavior configuration
//! - [`MouthConfig`] — talking behavior configuration
//!
//! ## Example
//!
//! ```ignore
//! EmotivaRig(
//!     layers: [
//!         (
//!             name: "base",
//!             image: "body.png",
//!             z_index: 0,
//!         ),
//!         (
//!             name: "eyes",
//!             image: "eyes_open.png",
//!             z_index: 10,
//!         ),
//!     ],
//!     eyes: Some((
//!         blink_interval_range: (2.0, 5.0),
//!         blink_duration: 0.12,
//!     )),
//! )
//! ```
//!
//! These structures are typically loaded from `.ron` files and passed to
//! the runtime using [`crate::Emotiva::from_rig`] or [`crate::Emotiva::from_ron_str`].

use crate::core::easing::Easing;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Lean {
    /// Maximum rotation in degrees (e.g. 5.0 = tilt ±5°)
    pub max_angle: f32,
}

/// Describes one-shot or directional motion animation.
#[derive(Debug, Clone, Deserialize)]
pub struct MotionDef {
    pub target: (f32, f32),
    pub duration: f32,
    #[serde(default)]
    pub easing: Option<Easing>,
    #[serde(default)]
    pub rotation: Option<f32>,
}

/// Describes tween animation for a layer.
#[derive(Debug, Clone, Deserialize)]
pub struct Tween {
    /// Amount of maximum offset movement (x, y)
    pub sway: (f32, f32),

    /// Optional lean (in degrees)
    #[serde(default)]
    pub lean: Option<Lean>,

    /// Duration in seconds for one complete sway loop
    pub period: f32,

    /// Optional easing curve for start tween motion
    #[serde(default)]
    pub easing_start: Option<Easing>,

    /// Optional easing curve for stop tween motion
    #[serde(default)]
    pub easing_stop: Option<Easing>,
}

/// A single image layer in a character rig.
#[derive(Debug, Clone, Deserialize)]
pub struct Layer {
    /// Logical name for the part (e.g. "eyes", "mouth", "base")
    pub name: String,

    /// Image filename or identifier
    pub image: String,

    /// Position offset (x, y) in pixels
    pub offset: Option<(f32, f32)>,

    /// Scale multiplier (1.0 = original size)
    pub scale: Option<f32>,

    /// Draw order (lower = behind, higher = in front)
    pub z_index: i32,

    /// Optional animation for this layer
    pub tween: Option<Tween>,

    /// Optional motion definition (used for Motion2D setup)
    #[serde(default)]
    pub motion: Option<MotionDef>,

    /// Optional list of alternative image filenames to swap into this layer
    pub variants: Option<HashMap<String, String>>,

    // Optional parent layer to inherit transform from
    pub parent: Option<String>,

    // Whether to inherit parent's transform (default: true)
    pub inherit: Option<bool>,
}

/// Blinking configuration for characters with animated eyes.
#[derive(Debug, Clone, Deserialize)]
pub struct EyesConfig {
    /// How often blinking can occur (range in seconds)
    pub blink_interval_range: (f32, f32),

    /// Duration of each blink (in seconds)
    pub blink_duration: f32,
}

/// Talking configuration for characters with animated mouths.
#[derive(Debug, Clone, Deserialize)]
pub struct MouthConfig {
    /// How long each talk session lasts (in seconds)
    pub talk_duration: f32,

    /// How long to wait before the next talk session starts
    pub talk_interval: f32,

    /// How long the mouth stays open and closed (in seconds)
    pub flap_time: f32,
}

/// A full character rig (body), consisting of multiple layers.
#[derive(Debug, Clone, Deserialize)]
pub struct EmotivaRig {
    /// All layers in this character, ordered arbitrarily
    pub layers: Vec<Layer>,

    /// Optional eye blink animation configuration
    #[serde(default)]
    pub eyes: Option<EyesConfig>,

    /// Optional mouth talking animation configuration
    #[serde(default)]
    pub mouth: Option<MouthConfig>,
}
