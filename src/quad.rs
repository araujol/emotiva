//! EmotivaQuad – Macroquad Integration Layer
//!
//! This module provides the integration layer between Emotiva's animation core and the Macroquad
//! rendering backend. It handles loading rig assets (images, layer definitions), managing the
//! animation loop, and rendering character rigs each frame.
//!
//! Features:
//! - Loads `.emotiva.ron` character rig files
//! - Applies per-frame animations (e.g. blinking, idle motion)
//! - Supports positioning characters via base offset (e.g. from VN engine)
//! - Designed to work independently from game/engine logic (e.g. Rusutori)
//!
//! This module is intended for use in applications using Macroquad to display expressive,
//! layered character animations in visual novels and similar 2D experiences.

use crate::CharAnimator;
use crate::format::load_rig_from_file;

// Use crate rand from root to avoid Macroquad's re-export conflict
use ::rand::rng;
use ::rand::rngs::ThreadRng;
use std::collections::HashMap;

use macroquad::prelude::*;

pub struct EmotivaQuad {
    animator: CharAnimator,
    textures: HashMap<String, Texture2D>,
    rng: ThreadRng,
    base_position: Vec2,
}

impl EmotivaQuad {
    pub async fn load(path: &str, texture_base_path: &str) -> Self {
        let rig = load_rig_from_file(path).expect("Failed to load .ron rig file");
        let mut rng = rng();
        let animator = CharAnimator::new(rig, &mut rng);

        let mut textures: HashMap<String, Texture2D> = HashMap::new();

        for layer in &animator.rig.layers {
            // Load base image
            if !textures.contains_key(&layer.image) {
                let tex = load_texture(&format!("{}/{}", texture_base_path, layer.image))
                    .await
                    .unwrap();
                textures.insert(layer.image.clone(), tex);
            }

            // Load variant images if present
            if let Some(variants) = &layer.variants {
                for image_path in variants.values() {
                    if !textures.contains_key(image_path) {
                        let tex = load_texture(&format!("{}/{}", texture_base_path, image_path))
                            .await
                            .unwrap();
                        textures.insert(image_path.clone(), tex);
                    }
                }
            }
        }

        EmotivaQuad {
            animator,
            textures,
            rng,
            base_position: Vec2::ZERO,
        }
    }

    pub fn set_base_position(&mut self, pos: Vec2) {
        self.base_position = pos;
    }

    pub fn set_image(&mut self, layer_name: &str, variant: &str) {
        self.animator.set_layer(layer_name, variant);
    }

    pub fn reset_image(&mut self, layer_name: &str) {
        self.animator.reset_layer(layer_name);
    }

    pub fn update(&mut self, dt: f32) {
        self.animator.update(dt, &mut self.rng);
    }

    pub fn draw(&mut self) {
        for sprite in self.animator.get_drawables() {
            if let Some(tex) = self.textures.get(&sprite.image) {
                let pos = vec2(sprite.position.0, sprite.position.1) + self.base_position;
                let size = vec2(tex.width() * sprite.scale, tex.height() * sprite.scale);

                let params = DrawTextureParams {
                    dest_size: Some(size),
                    rotation: sprite.rotation,
                    pivot: Some(vec2(tex.width() / 2.0, tex.height() / 2.0)),
                    ..Default::default()
                };

                draw_texture_ex(tex, pos.x, pos.y, WHITE, params);
            }
        }
    }

    pub fn trigger(&mut self, layer: &str, action: &str) {
        self.animator.trigger(layer, action);
    }

    pub fn is_tween_enabled(&mut self, layer: &str) -> bool {
        return self.animator.is_tween_enabled(layer);
    }

    pub fn is_motion_finished(&mut self, layer: &str) -> bool {
        return self.animator.is_motion_finished(layer);
    }
}
