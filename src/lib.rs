//! Emotiva Core - Animation runtime and data structures

pub mod format;
mod mouth;
pub mod quad;

use mouth::MouthState;
use rand::Rng;
use serde::Deserialize;

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
}

/// A full character rig, consisting of multiple layers.
#[derive(Debug, Clone, Deserialize)]
pub struct CharRig {
    /// All layers in this character, ordered arbitrarily
    pub layers: Vec<CharLayer>,
}

/// The result of a frame update: a layer with absolute transform info.
#[derive(Debug, Clone)]
pub struct DrawableSprite {
    pub image: String,
    pub position: (f32, f32),
    pub scale: f32,
    pub z_index: i32,
}

/// The character animator holds time state and generates drawables.
pub struct CharAnimator {
    pub rig: CharRig,
    pub time: f32,
    pub mouth: MouthState,
}

impl CharAnimator {
    pub fn new(rig: CharRig, rng: &mut impl Rng) -> Self {
        Self {
            rig,
            time: 0.0,
            mouth: MouthState::new(0.0, rng),
        }
    }

    /// Advance animation state by delta time (in seconds)
    pub fn update(&mut self, delta_time: f32, rng: &mut impl Rng) {
        self.time += delta_time;
        self.mouth.update(self.time, rng);
    }

    /// Returns transformed sprites to render this frame.
    pub fn get_drawables(&self) -> Vec<DrawableSprite> {
        let mut output = Vec::new();

        // Natural blink: every 4 seconds, blink lasts 0.15s
        let blink_interval = 4.0;
        let blink_duration = 0.15;
        let blink_phase = self.time % blink_interval;
        let is_blinking = blink_phase < blink_duration;

        for layer in &self.rig.layers {
            // Skip eyes_open if blinking
            if is_blinking && layer.name == "eyes" && layer.image.contains("eyes_open") {
                continue;
            }
            // Skip eyes_closed if not blinking
            if !is_blinking && layer.name == "eyes" && layer.image.contains("eyes_closed") {
                continue;
            }

            // Skip mouth_closed if mouth is open
            if self.mouth.is_open(self.time)
                && layer.name == "mouth"
                && layer.image.contains("mouth_closed")
            {
                continue;
            }
            // Skip mouth_open if mouth is closed
            if !self.mouth.is_open(self.time)
                && layer.name == "mouth"
                && layer.image.contains("mouth_open")
            {
                continue;
            }

            // Simple idle animation: sinusoidal Y breathing offset
            let breathing_offset = (self.time * 2.0).sin() * 2.0; // 2px up/down

            output.push(DrawableSprite {
                image: layer.image.clone(),
                position: (layer.offset.0, layer.offset.1 + breathing_offset),
                scale: layer.scale,
                z_index: layer.z_index,
            });
        }

        // Sort by z_index before drawing
        output.sort_by_key(|s| s.z_index);
        output
    }
}
