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
pub mod tween;

use anim::eyes::EyesState;
use anim::mouth::MouthState;
use format::{CharRig, Motion};
use tween::TweenState;

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
    pub tweens: Vec<TweenState>,
    pub mouth: Option<MouthState>,
    pub eyes: Option<EyesState>,
}

impl CharAnimator {
    pub fn new(rig: CharRig, rng: &mut impl Rng) -> Self {
        let mut tweens = Vec::new();

        // Initialize shared animation states (one for all eyes layers, one for all mouth layers)
        // so that "eyes_open" / "eyes_closed" and similar variants animate in sync.
        let has_mouth = rig
            .layers
            .iter()
            .any(|l| matches!(l.motion, Some(Motion::Mouth)));
        let has_eyes = rig
            .layers
            .iter()
            .any(|l| matches!(l.motion, Some(Motion::Blink)));

        let mouth = has_mouth.then(|| MouthState::new(0.0, rng));
        let eyes = has_eyes.then(|| EyesState::new(0.0, rng));

        for _ in &rig.layers {
            tweens.push(TweenState::new());
        }

        Self {
            rig,
            time: 0.0,
            tweens,
            mouth,
            eyes,
        }
    }

    /// Advance animation state by delta time (in seconds)
    pub fn update(&mut self, delta_time: f32, rng: &mut impl Rng) {
        self.time += delta_time;

        if let Some(mouth) = &mut self.mouth {
            mouth.update(self.time, rng);
        }

        if let Some(eye) = &mut self.eyes {
            eye.update(self.time, rng);
        }
    }

    /// Returns transformed sprites to render this frame.
    pub fn get_drawables(&mut self) -> Vec<DrawableSprite> {
        let mut output = Vec::new();

        // Skip drawing alternate eye/mouth layers that don't match current state.
        // This works in tandem with the shared EyesState and MouthState.
        // This avoids rendering both versions at once and keeps everything visually in sync.
        for (i, layer) in self.rig.layers.iter().enumerate() {
            if let Some(eye) = &self.eyes {
                if eye.is_blinking() && layer.image.contains("eyes_open") {
                    continue;
                }
                if !eye.is_blinking() && layer.image.contains("eyes_closed") {
                    continue;
                }
            }

            if let Some(mouth) = &self.mouth {
                // Skip mouth_closed if mouth is open
                if mouth.is_open(self.time) && layer.image.contains("mouth_closed") {
                    continue;
                }
                // Skip mouth_open if mouth is closed
                if !mouth.is_open(self.time) && layer.image.contains("mouth_open") {
                    continue;
                }
            }

            let mut offset = layer.offset;

            if let Some(tween) = &layer.tween {
                let tween_state = &mut self.tweens[i];
                let tween_offset = tween_state.update(1.0 / 60.0, tween); // Assume 60fps tick size
                offset.0 += tween_offset.dx;
                offset.1 += tween_offset.dy;
            }

            output.push(DrawableSprite {
                image: layer.image.clone(),
                position: offset,
                scale: layer.scale,
                z_index: layer.z_index,
            });
        }

        // Sort by z_index before drawing
        output.sort_by_key(|s| s.z_index);
        output
    }
}
