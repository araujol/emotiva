//! # 🎭 Emotiva Runtime
//!
//! High-level runtime context for **Emotiva character animation**.
//!
//! The [`Emotiva`] struct acts as the main runtime container used by
//! frontend applications. It owns an [`EmotivaHeart`] instance and
//! coordinates animation updates, state management, and optional
//! rendering integrations.
//!
//! The runtime itself is **renderer-agnostic**, meaning it can operate
//! without any graphics backend. When the `macroquad` feature is enabled,
//! additional helper functions are provided for loading textures and
//! rendering characters directly.
//!
//! ## Responsibilities
//!
//! The runtime is responsible for:
//!
//! - Owning and updating the internal [`EmotivaHeart`] animation engine
//! - Advancing animation state each frame
//! - Producing resolved [`EmotivaForm`] values for rendering
//! - Managing optional frontend integrations (e.g. Macroquad)
//! - Providing save/load snapshot helpers
//! - Forwarding the public animation API through [`crate::api::EmotivaAPI`]
//!
//! ## Basic Usage
//!
//! ```ignore
//! let mut emo = Emotiva::from_ron_str(rig_data);
//!
//! loop {
//!     emo.update(dt);
//!
//!     for form in emo.forms() {
//!         // render sprite using your engine
//!     }
//! }
//! ```
//!
//! ## Rendering Backends
//!
//! Emotiva does not require a renderer. Applications are free to draw the
//! returned [`EmotivaForm`] values using any graphics system.
//!
//! When the `macroquad` feature is enabled, helper utilities are provided
//! to load textures and render characters directly using Macroquad.
//!
//! ## Architecture
//!
//! ```text
//! Rig (.ron)
//!      │
//!      ▼
//!  EmotivaRuntime
//!      │
//!      ▼
//!  EmotivaHeart
//!      │
//!      ▼
//!  EmotivaForm list
//!      │
//!      ▼
//! Rendering backend (Macroquad, Bevy, custom engine)
//! ```

use crate::core::easing::Easing;
use crate::format::EmotivaRig;
use crate::snapshot::EmotivaSnapshot;
use crate::{EmotivaForm, EmotivaHeart};
use ron::de::from_str as ron_from_str;

// RNG: native uses ThreadRng; wasm uses a seeded ChaCha8 (no OS entropy)
#[cfg(not(target_arch = "wasm32"))]
use ::rand::{rng, rngs::ThreadRng};
#[cfg(target_arch = "wasm32")]
use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};
#[cfg(feature = "macroquad")]
use std::collections::HashMap;

/// Core Emotiva animation context.
///
/// This type owns the animation heart and optional texture storage. It can be
/// updated and queried each frame to obtain resolved `EmotivaForm`s. Rendering
/// is performed only when a frontend (such as Macroquad) is enabled.
pub struct Emotiva {
    pub heart: EmotivaHeart,
    // Position relative to screen size
    pub screen_position: (f32, f32),
    #[cfg(not(target_arch = "wasm32"))]
    rng: ThreadRng,
    #[cfg(target_arch = "wasm32")]
    rng: ChaCha8Rng,
    #[cfg(feature = "macroquad")]
    textures: HashMap<String, macroquad::prelude::Texture2D>,
}

impl Emotiva {
    /// Creates an `Emotiva` instance from a pre-parsed rig.
    pub fn from_rig(rig: EmotivaRig) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let rng = rng();
        #[cfg(target_arch = "wasm32")]
        let rng = ChaCha8Rng::seed_from_u64(12345);

        let heart = EmotivaHeart::new(rig);

        Emotiva {
            heart,
            screen_position: (0.0, 0.0),
            #[cfg(feature = "macroquad")]
            textures: HashMap::new(),
            rng,
        }
    }

    /// Creates an `Emotiva` instance from a RON string.
    pub fn from_ron_str(ron: &str) -> Self {
        let rig: EmotivaRig = ron_from_str(ron).expect("Failed to parse Emotiva rig RON");
        Self::from_rig(rig)
    }

    /// Advances animation state by `dt` seconds.
    pub fn update(&mut self, dt: f32) {
        self.heart.update(dt, &mut self.rng);
    }

    /// Returns the resolved visual forms for the current frame.
    pub fn forms(&mut self) -> Vec<EmotivaForm> {
        self.heart.get_forms()
    }

    /// Set the position relative to screen coordinates.
    pub fn set_screen_position(&mut self, pos: (f32, f32)) {
        self.screen_position = pos;
    }

    /// Returns the current screen/stage position offset.
    pub fn screen_position(&self) -> (f32, f32) {
        self.screen_position
    }

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

    pub fn save_state(&self) -> EmotivaSnapshot {
        self.save_snapshot()
    }

    pub fn load_state(&mut self, snap: EmotivaSnapshot) {
        self.load_snapshot(snap);
    }
}

// ============= Macroquad frontend (feature-gated) ============= //

#[cfg(feature = "macroquad")]
mod macroquad_impl {
    use super::*;
    use macroquad::prelude::*;

    impl Emotiva {
        pub async fn from_ron_file(path: &str) -> Self {
            let ron = macroquad::file::load_string(path)
                .await
                .expect("Failed to load Emotiva rig file");

            Self::from_ron_str(&ron)
        }

        pub async fn load_with_textures(path: &str, texture_base_path: &str) -> Self {
            // 1) Load rig via Macroquad IO, then build Emotiva core
            let mut emo = Self::from_ron_file(path).await;

            // 2) Load textures referenced by the rig
            let mut textures: HashMap<String, Texture2D> = HashMap::new();

            for layer in &emo.heart.rig.layers {
                if !textures.contains_key(&layer.image) {
                    let tex = load_texture(&format!("{}/{}", texture_base_path, layer.image))
                        .await
                        .unwrap();
                    textures.insert(layer.image.clone(), tex);
                }

                if let Some(variants) = &layer.variants {
                    for image_path in variants.values() {
                        if !textures.contains_key(image_path) {
                            let tex =
                                load_texture(&format!("{}/{}", texture_base_path, image_path))
                                    .await
                                    .unwrap();
                            textures.insert(image_path.clone(), tex);
                        }
                    }
                }
            }

            // 3) Attach textures to the runtime
            emo.textures = textures;
            emo
        }

        /// Draws the current frame using Macroquad.
        pub fn draw(&mut self) {
            for sprite in self.heart.get_forms() {
                if let Some(tex) = self.textures.get(&sprite.image) {
                    let pivot = vec2(tex.width() / 2.0, tex.height() / 2.0);
                    let pos = vec2(sprite.position.0, sprite.position.1)
                        + vec2(self.screen_position().0, self.screen_position().1)
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
    }
}

// ============= EmotivaAPI ============= //
use crate::api::EmotivaAPI;
// Macros used to forward methods from EmotivaAPI to Emotiva
use crate::{impl_fns_mut, impl_fns_ref};

impl EmotivaAPI for Emotiva {
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
        fn is_motion_playing(&self, layer: &str) -> bool;
        fn is_rotation_playing(&self, layer: &str) -> bool;
        fn is_tween_enabled(&self, layer: &str) -> bool;
        fn is_tween_paused(&self, layer: &str) -> bool;
        fn is_scale_finished(&self, layer: &str) -> bool;
        fn is_alpha_finished(&self, layer: &str) -> bool;
        fn is_tint_finished(&self, layer: &str) -> bool;
        }
    }
}
