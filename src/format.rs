//! Emotiva Format – Rig File Definition & Loader
//!
//! This module defines the serializable data structures (`CharRig`, `CharLayer`) used in `.emotiva.ron` files,
//! and provides functions to load them from disk.
//!
//! Features:
//! - `CharRig`: Top-level character rig containing ordered layers
//! - `CharLayer`: Describes an image layer with offset, scale, and z-index
//! - Rig loading from `.ron` files using Serde and error handling
//!
//! This is the bridge between static character definitions and the animation engine.

use crate::core::easing::Easing;
use ron::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur while loading a character rig file.
#[derive(Debug, Error)]
pub enum RigLoadError {
    #[error("Failed to open file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
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

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Lean {
    /// Maximum rotation in degrees (e.g. 5.0 = tilt ±5°)
    pub max_angle: f32,
}

/// A single image layer in a character rig.
#[derive(Debug, Clone, Deserialize)]
pub struct CharLayer {
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

/// A full character rig, consisting of multiple layers.
#[derive(Debug, Clone, Deserialize)]
pub struct CharRig {
    /// All layers in this character, ordered arbitrarily
    pub layers: Vec<CharLayer>,

    /// Optional eye blink animation configuration
    #[serde(default)]
    pub eyes: Option<EyesConfig>,

    /// Optional mouth talking animation configuration
    #[serde(default)]
    pub mouth: Option<MouthConfig>,
}

/// Loads a character rig from a `.ron` file path in async mode.
pub async fn load_rig_from_file(path: &str) -> Result<CharRig, RigLoadError> {
    let contents = macroquad::file::load_string(path).await.map_err(|e| {
        use std::io::{Error as IoError, ErrorKind};
        RigLoadError::Io(IoError::new(ErrorKind::Other, e.to_string()))
    })?;
    let rig: CharRig = from_str(&contents)?;
    Ok(rig)
}
