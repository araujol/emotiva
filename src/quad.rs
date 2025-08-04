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

use crate::EmotivaHeart;
use crate::easing::Easing;
use crate::format::load_rig_from_file;

// Use crate rand from root to avoid Macroquad's re-export conflict
use ::rand::rng;
use ::rand::rngs::ThreadRng;
use std::collections::HashMap;

use macroquad::prelude::*;

pub struct EmotivaQuad {
    heart: EmotivaHeart,
    textures: HashMap<String, Texture2D>,
    rng: ThreadRng,
    base_position: Vec2,
}

impl EmotivaQuad {
    pub async fn load(path: &str, texture_base_path: &str) -> Self {
        let rig = load_rig_from_file(path).expect("Failed to load .ron rig file");
        let mut rng = rng();
        let heart = EmotivaHeart::new(rig, &mut rng);

        let mut textures: HashMap<String, Texture2D> = HashMap::new();

        for layer in &heart.rig.layers {
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
            heart,
            textures,
            rng,
            base_position: Vec2::ZERO,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.heart.update(dt, &mut self.rng);
    }

    pub fn draw(&mut self) {
        for sprite in self.heart.get_drawables() {
            if let Some(tex) = self.textures.get(&sprite.image) {
                // Adjust position so scaling/rotation happen around the sprite's center.
                // Subtracting the scaled pivot ensures correct alignment at the intended position.
                let pivot = vec2(tex.width() / 2.0, tex.height() / 2.0);
                let pos = vec2(sprite.position.0, sprite.position.1) + self.base_position
                    - pivot * sprite.scale;

                let size = vec2(tex.width(), tex.height()) * sprite.scale;

                let params = DrawTextureParams {
                    dest_size: Some(size),
                    rotation: sprite.rotation,
                    pivot: Some(pivot),
                    ..Default::default()
                };

                draw_texture_ex(
                    tex,
                    pos.x,
                    pos.y,
                    Color::new(
                        sprite.tint[0],
                        sprite.tint[1],
                        sprite.tint[2],
                        sprite.alpha * sprite.tint[3],
                    ),
                    params,
                );
            }
        }
    }

    // =============== EmotivaQuad API Methods =============== //
    pub fn set_base_position(&mut self, pos: Vec2) {
        self.base_position = pos;
    }

    /* In order for callbacks to be nested, they need to be directly
    forwarded to the EmotivaHeart type */
    pub fn on_start<F>(&mut self, id: u64, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        self.heart.on_start(id, cb);
    }

    pub fn on_end<F>(&mut self, id: u64, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        self.heart.on_end(id, cb);
    }

    pub fn on_delay<F>(&mut self, duration: f32, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        self.heart.on_delay(duration, cb);
    }
}

// ============= EmmotivaAPI ============= //
use crate::api::EmotivaAPI;
// Macros used to forward methods from EmotivaAPI to EmotivaQuad
use crate::{impl_fns_mut, impl_fns_ref};

impl EmotivaAPI for EmotivaQuad {
    // Mutable methods
    impl_fns_mut! {
        heart => {
        fn trigger(&mut self, layer: &str, action: &str);
        fn set_layer(&mut self, layer_name: &str, variant: &str);
        fn reset_layer(&mut self, layer_name: &str);
        fn motion_play(&mut self, layer: &str) -> u64;
        fn motion_reverse(&mut self, layer: &str) -> u64;
        fn rotation_play(&mut self, layer: &str) -> u64;
        fn rotation_reverse(&mut self, layer: &str) -> u64;
        fn tween_start(&mut self, layer: &str) -> u64;
        fn tween_stop(&mut self, layer: &str);
        fn tween_start_easing(&mut self, layer: &str) -> u64;
        fn tween_stop_easing(&mut self, layer: &str);
        fn tween_pause(&mut self, layer: &str);
        fn tween_resume(&mut self, layer: &str);
        fn set_scale(&mut self, layer: &str, min: f32, max: f32, speed: f32, easing: Easing)
        -> u64;
        fn remove_scale(&mut self, layer: &str);
        fn set_alpha(&mut self, layer: &str, from: f32, to: f32, speed: f32, easing: Easing)
        -> u64;
        fn remove_alpha(&mut self, layer: &str);
        fn set_tint(&mut self, layer: &str, from: [f32; 4], to: [f32; 4], duration: f32, easing: Easing) -> u64;
        fn remove_tint(&mut self, layer: &str);
        fn clear_all_fx(&mut self);
        fn set_delay(&mut self, duration: f32) -> u64;
        }
    }

    // Immutable methods
    impl_fns_ref! {
        heart => {
        fn is_motion_finished(&self, layer: &str) -> bool;
        fn is_rotation_finished(&self, layer: &str) -> bool;
        fn is_tween_enabled(&self, layer: &str) -> bool;
        fn is_tween_paused(&self, layer: &str) -> bool;
        }
    }
}
