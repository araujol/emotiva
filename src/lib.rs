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
pub mod easing;
pub mod events;
pub mod format;
pub mod fx;
pub mod motion;
pub mod quad;
pub mod transform;
pub mod tween;

pub mod palette;

use anim::eyes::EyesState;
use anim::mouth::MouthState;
use format::CharRig;
use fx::VisualFxState;
use motion::{Motion2D, Rotation};
use tween::TweenState;

use rand::Rng;
use std::collections::HashMap;

use crate::easing::Easing;

/// The result of a frame update: a layer with absolute transform info.
#[derive(Debug, Clone)]
pub struct DrawableSprite {
    pub image: String,
    pub position: (f32, f32),
    pub scale: f32,
    pub rotation: f32,
    pub z_index: i32,
    pub alpha: f32,
    pub tint: [f32; 4],
}

/// The character animator holds time state and generates drawables.
pub struct CharAnimator {
    pub rig: CharRig,
    pub time: f32,
    pub tweens: Vec<TweenState>,
    pub mouth: Option<MouthState>,
    pub eyes: Option<EyesState>,
    pub image_overrides: HashMap<String, String>, // layer name -> image override
    pub image_variants: HashMap<String, HashMap<String, String>>, // layer -> variant_name -> image
    pub motions: HashMap<String, Motion2D>,       // layer name -> motion animation
    pub rotations: HashMap<String, Rotation>,
    pub visual_fx: VisualFxState,
}

impl CharAnimator {
    pub fn new(rig: CharRig, rng: &mut impl Rng) -> Self {
        let mut tweens = Vec::new();
        let mut motions = HashMap::new();
        let mut rotations = HashMap::new();

        let has_mouth = rig.layers.iter().any(|l| l.name.contains("mouth"));
        let has_eyes = rig.layers.iter().any(|l| l.name.contains("eyes"));

        let mouth = has_mouth.then(|| MouthState::new(0.0, rng));
        let eyes = has_eyes.then(|| EyesState::new(0.0, rng));

        for (_i, layer) in rig.layers.iter().enumerate() {
            tweens.push(TweenState::new());

            if let Some(def) = &layer.motion {
                motions.insert(
                    layer.name.clone(),
                    Motion2D::new(
                        (0.0, 0.0),
                        def.target,
                        def.duration,
                        def.easing.unwrap_or(Easing::Linear),
                    ),
                );

                if let Some(deg) = def.rotation {
                    rotations.insert(
                        layer.name.clone(),
                        Rotation::new(deg, def.duration, def.easing.unwrap_or(Easing::Linear)),
                    );
                }
            }
        }

        let mut image_variants = HashMap::new();

        for layer in &rig.layers {
            let mut variant_map = HashMap::new();
            if let Some(variants) = &layer.variants {
                for (variant_name, image_name) in variants {
                    variant_map.insert(variant_name.clone(), image_name.clone());
                }
            }
            image_variants.insert(layer.name.clone(), variant_map);
        }

        Self {
            rig,
            tweens,
            mouth,
            eyes,
            image_variants,
            motions,
            rotations,
            time: 0.0,
            image_overrides: HashMap::new(),
            visual_fx: VisualFxState::new(),
        }
    }

    /// Change a layer's image by name.
    pub fn set_layer(&mut self, layer_name: &str, variant: &str) {
        if let Some(layer_variants) = self.image_variants.get(layer_name) {
            if let Some(image_name) = layer_variants.get(variant) {
                self.image_overrides
                    .insert(layer_name.to_string(), image_name.clone());
            } else {
                eprintln!(
                    "Warning: unknown variant '{}' for layer '{}'",
                    variant, layer_name
                );
            }
        } else {
            eprintln!("Warning: layer '{}' has no image variants", layer_name);
        }
    }

    /// Reset a layer's image override back to the default.
    pub fn reset_layer(&mut self, layer_name: &str) {
        self.image_overrides.remove(layer_name);
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

        for tween in self.tweens.iter_mut() {
            tween.update(delta_time);
        }

        for motion in self.motions.values_mut() {
            motion.update(delta_time);
        }

        for rotation in self.rotations.values_mut() {
            rotation.update(delta_time);
        }

        self.visual_fx.update(delta_time);
    }

    /// Returns transformed sprites to render this frame.
    pub fn get_drawables(&mut self) -> Vec<DrawableSprite> {
        use crate::transform::resolve_all_transforms;

        let mut output = Vec::new();
        let transforms = resolve_all_transforms(
            &self.rig,
            &mut self.tweens,
            &self.motions,
            &self.rotations,
            &self.visual_fx,
        );

        for layer in &self.rig.layers {
            // Skip eye and mouth conditions
            if let Some(eye) = &self.eyes {
                if eye.is_blinking() && layer.image.contains("eyes_open") {
                    continue;
                }
                if !eye.is_blinking() && layer.image.contains("eyes_closed") {
                    continue;
                }
            }

            if let Some(mouth) = &self.mouth {
                if mouth.is_open(self.time) && layer.image.contains("mouth_closed") {
                    continue;
                }
                if !mouth.is_open(self.time) && layer.image.contains("mouth_open") {
                    continue;
                }
            }

            let fallback = &transform::WorldTransform::default();
            let transform = transforms.get(&layer.name).unwrap_or(fallback);

            let final_image = self
                .image_overrides
                .get(&layer.name)
                .cloned()
                .unwrap_or_else(|| layer.image.clone());

            output.push(DrawableSprite {
                image: final_image,
                position: (transform.position.x, transform.position.y),
                scale: transform.scale,
                rotation: transform.rotation,
                z_index: layer.z_index,
                alpha: transform.alpha,
                tint: transform.tint,
            });
        }

        // Sort by z_index before drawing
        output.sort_by_key(|s| s.z_index);
        output
    }

    pub fn trigger(&mut self, layer: &str, action: &str) {
        match (layer, action) {
            ("eyes", "start_blinking") => {
                if let Some(eyes) = &mut self.eyes {
                    eyes.start();
                }
            }
            ("eyes", "stop_blinking") => {
                if let Some(eyes) = &mut self.eyes {
                    eyes.stop();
                }
            }
            ("mouth", "start_talking") => {
                if let Some(mouth) = &mut self.mouth {
                    mouth.start();
                }
            }
            ("mouth", "stop_talking") => {
                if let Some(mouth) = &mut self.mouth {
                    mouth.stop();
                }
            }
            ("mouth", "idle_chat") => {
                if let Some(mouth) = &mut self.mouth {
                    mouth.idle_chat();
                }
            }
            _ => {
                eprintln!("Unknown trigger: {}/{}", layer, action);
            }
        }
    }

    // Motion API
    pub fn motion_play(&mut self, layer: &str) {
        if let Some(motion) = self.motions.get_mut(layer) {
            motion.play();
        }
    }

    pub fn motion_reverse(&mut self, layer: &str) {
        if let Some(motion) = self.motions.get_mut(layer) {
            motion.reverse();
        }
    }

    pub fn rotation_play(&mut self, layer: &str) {
        if let Some(rotation) = self.rotations.get_mut(layer) {
            rotation.play();
        }
    }

    pub fn rotation_reverse(&mut self, layer: &str) {
        if let Some(rotation) = self.rotations.get_mut(layer) {
            rotation.reverse();
        }
    }

    pub fn is_motion_finished(&mut self, layer: &str) -> bool {
        let motion_done = self
            .motions
            .get_mut(layer)
            .map(|m| m.is_finished())
            .unwrap_or(true);
        motion_done
    }

    pub fn is_rotation_finished(&mut self, layer: &str) -> bool {
        let rotation_done = self
            .rotations
            .get_mut(layer)
            .map(|r| r.is_finished())
            .unwrap_or(true);
        rotation_done
    }

    // Tween system API
    pub fn tween_start(&mut self, layer: &str) {
        if let Some(index) = self.rig.layers.iter().position(|l| l.name == layer) {
            self.tweens[index].start();
        }
    }

    pub fn tween_stop(&mut self, layer: &str) {
        if let Some(index) = self.rig.layers.iter().position(|l| l.name == layer) {
            self.tweens[index].stop();
        }
    }

    pub fn tween_start_easing(&mut self, layer: &str) {
        if let Some(index) = self.rig.layers.iter().position(|l| l.name == layer) {
            if let Some(tween) = &self.rig.layers[index].tween {
                self.tweens[index].start_easing(tween);
            }
        }
    }

    pub fn tween_stop_easing(&mut self, layer: &str) {
        if let Some(index) = self.rig.layers.iter().position(|l| l.name == layer) {
            if let Some(tween) = &self.rig.layers[index].tween {
                self.tweens[index].stop_easing(tween);
            }
        }
    }

    pub fn tween_pause(&mut self, layer: &str) {
        if let Some(index) = self.rig.layers.iter().position(|l| l.name == layer) {
            self.tweens[index].pause();
        }
    }

    pub fn tween_resume(&mut self, layer: &str) {
        if let Some(index) = self.rig.layers.iter().position(|l| l.name == layer) {
            self.tweens[index].resume();
        }
    }

    pub fn is_tween_enabled(&mut self, layer: &str) -> bool {
        if let Some(index) = self.rig.layers.iter().position(|l| l.name == layer) {
            self.tweens[index].is_enabled()
        } else {
            false
        }
    }

    pub fn is_tween_paused(&mut self, layer: &str) -> bool {
        if let Some(index) = self.rig.layers.iter().position(|l| l.name == layer) {
            self.tweens[index].is_paused()
        } else {
            false
        }
    }

    // FX API functions
    pub fn set_scale(&mut self, layer: &str, from: f32, to: f32, duration: f32, easing: Easing) {
        self.visual_fx
            .add_scale_fx(layer, crate::fx::make_scale_fx(from, to, duration, easing));
    }

    pub fn remove_scale(&mut self, layer: &str) {
        self.visual_fx.remove_scale_fx(layer);
    }

    pub fn set_alpha(&mut self, layer: &str, from: f32, to: f32, duration: f32, easing: Easing) {
        self.visual_fx
            .add_alpha_fx(layer, crate::fx::make_alpha_fx(from, to, duration, easing));
    }

    pub fn remove_alpha(&mut self, layer: &str) {
        self.visual_fx.remove_alpha_fx(layer);
    }

    // Clear all FX
    pub fn clear_all_fx(&mut self) {
        self.visual_fx.clear_all_fx();
    }

    // Color Tint methods
    pub fn set_tint(
        &mut self,
        layer: &str,
        from: [f32; 4],
        to: [f32; 4],
        duration: f32,
        easing: Easing,
    ) {
        self.visual_fx
            .add_tint_fx(layer, crate::fx::make_tint_fx(from, to, duration, easing));
    }

    pub fn remove_tint(&mut self, layer: &str) {
        self.visual_fx.remove_tint_fx(layer);
    }
}
