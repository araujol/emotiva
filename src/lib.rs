//! Emotiva Core - Animation runtime and data structures
//!
//! This library powers expressive 2D character animation for visual novels and games.
//! It provides a modular system for loading character rigs (.ron format), managing per-layer
//! motion and expression (e.g. blinking, mouth movement), and outputting drawable sprites
//! for use in rendering engines like Macroquad.
//!
//! Modules:
//! - `format`: Loader for .ron rig files
//! - `anim`: Subsystems for procedural body parts behavior
//! - `quad`: Optional macroquad rendering adapter
//!
//! Designed to integrate smoothly with Rusutori and other VN engines.

pub mod anim;
pub mod format;
pub mod quad;

use anim::eyes::EyesState;
use anim::mouth::MouthState;
use format::CharRig;

use rand::Rng;

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
    pub eyes: EyesState,
}

impl CharAnimator {
    pub fn new(rig: CharRig, rng: &mut impl Rng) -> Self {
        Self {
            rig,
            time: 0.0,
            mouth: MouthState::new(0.0, rng),
            eyes: EyesState::new(0.0, rng),
        }
    }

    /// Advance animation state by delta time (in seconds)
    pub fn update(&mut self, delta_time: f32, rng: &mut impl Rng) {
        self.time += delta_time;
        self.mouth.update(self.time, rng);
        self.eyes.update(self.time, rng);
    }

    /// Returns transformed sprites to render this frame.
    pub fn get_drawables(&self) -> Vec<DrawableSprite> {
        let mut output = Vec::new();

        let is_blinking = self.eyes.is_blinking();

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
