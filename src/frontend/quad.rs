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

use crate::core::easing::Easing;
use crate::format::CharRig;
use crate::format::load_rig_from_file;
use crate::{DrawableSprite, EmotivaHeart};
use ron::de::from_str as ron_from_str;

// RNG: native uses ThreadRng; wasm uses a seeded ChaCha8 (no OS entropy)
#[cfg(not(target_arch = "wasm32"))]
use ::rand::{rng, rngs::ThreadRng};
#[cfg(target_arch = "wasm32")]
use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};
use std::collections::HashMap;

use macroquad::prelude::*;

pub struct EmotivaQuad {
    heart: EmotivaHeart,
    textures: HashMap<String, Texture2D>,
    #[cfg(not(target_arch = "wasm32"))]
    rng: ThreadRng,
    #[cfg(target_arch = "wasm32")]
    rng: ChaCha8Rng,
    base_position: Vec2,
}

impl EmotivaQuad {
    /// Creates an `EmotivaQuad` from a pre-parsed rig RON string and preloaded textures (synchronous).
    pub fn from_ron_str(ron: &str, textures: HashMap<String, Texture2D>) -> Self {
        let rig: CharRig = ron_from_str(ron).expect("Failed to parse Emotiva rig RON");
        Self::from_rig(rig, textures)
    }

    /// Creates an `EmotivaQuad` using the rig file and preloaded textures.
    pub async fn from_rig_file(path: &str, textures: HashMap<String, Texture2D>) -> Self {
        let rig = load_rig_from_file(path)
            .await
            .expect("Failed to load .ron rig file");
        Self::from_rig(rig, textures)
    }

    /// Creates an `EmotivaQuad` from a pre-parsed rig and preloaded textures (synchronous).
    pub fn from_rig(rig: CharRig, textures: HashMap<String, Texture2D>) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let rng = rng();
        #[cfg(target_arch = "wasm32")]
        let rng = ChaCha8Rng::seed_from_u64(12345);

        // Create heart
        let heart = EmotivaHeart::new(rig);

        // (Debug) validate required textures are present
        #[cfg(debug_assertions)]
        {
            for layer in &heart.rig.layers {
                if !textures.contains_key(&layer.image) {
                    info!(
                        "[EmotivaQuad] Missing texture for base image '{}'",
                        layer.image
                    );
                }
                if let Some(variants) = &layer.variants {
                    for image_path in variants.values() {
                        if !textures.contains_key(image_path) {
                            info!(
                                "[EmotivaQuad] Missing texture for variant image '{}'",
                                image_path
                            );
                        }
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

    // Create an `EmotivaQuad` object using the rig file and textures path.
    // This function loads the textures directly.
    pub async fn load_from_path(path: &str, texture_base_path: &str) -> Self {
        let rig = load_rig_from_file(path)
            .await
            .expect("Failed to load .ron rig file");
        #[cfg(not(target_arch = "wasm32"))]
        let rng = rng();
        #[cfg(target_arch = "wasm32")]
        let rng = ChaCha8Rng::seed_from_u64(12345);
        let heart = EmotivaHeart::new(rig);

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

    /// Return the list of drawable sprites for the current frame.
    ///
    /// This exposes Emotiva’s **resolved, per-frame visual output** without
    /// performing any rendering. The returned `DrawableSprite`s contain
    /// fully evaluated transforms (position, scale, rotation, alpha, tint)
    /// derived from the rig and all active animations.
    pub fn drawables(&mut self) -> Vec<DrawableSprite> {
        self.heart.get_drawables()
    }

    /// Returns a read-only view of textures owned by this EmotivaQuad.
    pub fn textures(&self) -> &HashMap<String, Texture2D> {
        &self.textures
    }

    /// Convenient function to set the drawing position of the object.
    /// This method doesn't affect the drawable sprites state.
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
        fn eyes_start(&mut self) -> Option<u64>;
        fn eyes_stop(&mut self);
        fn eyes_set_blink_duration(&mut self, duration: f32);
        fn eyes_set_blink_interval_range(&mut self, range: (f32, f32));
        fn mouth_start(&mut self) -> Option<u64>;
        fn mouth_stop(&mut self);
        fn mouth_set_talk_interval(&mut self, interval: f32);
        fn mouth_set_talk_duration(&mut self, duration: f32);
        fn mouth_set_flap_open_time(&mut self, duration: f32);
        fn set_layer(&mut self, layer_name: &str, variant: &str);
        fn reset_layer(&mut self, layer_name: &str);
        fn motion_play(&mut self, layer: &str) -> Option<u64>;
        fn motion_reverse(&mut self, layer: &str) -> Option<u64>;
        fn rotation_play(&mut self, layer: &str) -> Option<u64>;
        fn rotation_reverse(&mut self, layer: &str) -> Option<u64>;
        fn tween_start(&mut self, layer: &str) -> Option<u64>;
        fn tween_stop(&mut self, layer: &str);
        fn tween_pause(&mut self, layer: &str);
        fn tween_resume(&mut self, layer: &str);
        fn set_scale(&mut self, layer: &str, min: f32, max: f32, speed: f32, easing: Easing) -> u64;
        fn remove_scale(&mut self, layer: &str);
        fn set_alpha(&mut self, layer: &str, from: f32, to: f32, speed: f32, easing: Easing) -> u64;
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
        fn eyes_is_blinking(&self) -> bool;
        fn eyes_is_blinking_enabled(&self) -> bool;
        fn mouth_is_talking(&self) -> bool;
        fn mouth_is_talking_enabled(&self) -> bool;
        fn is_motion_finished(&self, layer: &str) -> bool;
        fn is_rotation_finished(&self, layer: &str) -> bool;
        fn is_tween_enabled(&self, layer: &str) -> bool;
        fn is_tween_paused(&self, layer: &str) -> bool;
        }
    }
}
