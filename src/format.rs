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

use crate::easing::Easing;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
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
    pub offset: (f32, f32),

    /// Scale multiplier (1.0 = original size)
    pub scale: f32,

    /// Draw order (lower = behind, higher = in front)
    pub z_index: i32,

    /// Optional animation for this layer
    pub tween: Option<Tween>,

    /// Optional motion definition (used for Motion2D setup)
    #[serde(default)]
    pub motion: Option<MotionDef>,

    /// Optional list of alternative image filenames to swap into this layer
    pub variants: Option<HashMap<String, String>>,
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

/// A full character rig, consisting of multiple layers.
#[derive(Debug, Clone, Deserialize)]
pub struct CharRig {
    /// All layers in this character, ordered arbitrarily
    pub layers: Vec<CharLayer>,
}

/// Loads a character rig from a `.ron` file path.
pub fn load_rig_from_file<P: AsRef<Path>>(path: P) -> Result<CharRig, RigLoadError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let rig = from_reader(reader)?;
    Ok(rig)
}
