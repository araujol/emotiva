//! ==========================================
//! 🎭 **EmotivaHeart** – The Beating Heart of Emotiva
//! ------------------------------------------
//! This is the *central runtime* of Emotiva — the part that gives life
//! to every character rig, drives animations, and produces the drawables
//! you see on screen.
//!
//! ✅ **What EmotivaHeart Does:**
//! - Loads character rigs from `.ron` files (sprites, layers, variants)
//! - Holds the state for all animation systems (tweens, motions, rotations, FX)
//! - Updates these systems each frame to keep characters “alive”
//! - Outputs `DrawableSprite` structs ready for any rendering backend
//!
//! ✅ **What EmotivaHeart Doesn’t Do:**
//! - Doesn’t render directly (frontends like `emotiva-macroquad` handle that)
//! - Doesn’t expose every helper API itself (see `api/` modules for public API)
//!
//! 📦 **Why the name “Heart”?**
//! Because this struct isn’t just a “core” — it’s the *heartbeat* of Emotiva.
//! It pumps movement, expression, and emotion through the entire system.
//!
//! Designed to integrate smoothly with **Rusutori** and other visual novel engines.
//! ==========================================

pub mod anim;
pub mod api;
pub mod core;
pub mod format;
pub mod macros;
pub mod snapshot;

pub mod emotiva;
// Re-export the main runtime type at the crate root
pub use emotiva::Emotiva;

use anim::eyes::EyesState;
use anim::mouth::MouthState;
use core::delay::Delay;
use core::fx::VisualFxState;
use core::motion::{Motion2D, Rotation};
use core::transform::{WorldTransform, resolve_all_transforms};
use core::tween::TweenState;
use format::CharRig;

use rand::Rng;
use std::collections::HashMap;

use crate::core::easing::Easing;
use crate::core::events::AnimEvent;

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

/// **EmotivaHeart** – the main runtime struct managing all animation state.
///
/// - Owns the loaded `CharRig` (layer data, variants)
/// - Tracks all tweens, motions, rotations, and FX
/// - Produces `DrawableSprite`s for rendering each frame
pub struct EmotivaHeart {
    pub rig: CharRig,
    pub time: f32,
    pub mouth: Option<MouthState>,
    pub eyes: Option<EyesState>,
    pub image_overrides: HashMap<String, String>, // layer name -> image override
    pub image_variants: HashMap<String, HashMap<String, String>>, // layer -> variant_name -> image
    pub tweens: HashMap<String, TweenState>,
    pub motions: HashMap<String, Motion2D>, // layer name -> motion animation
    pub rotations: HashMap<String, Rotation>,
    pub visual_fx: VisualFxState,
    /// Global delays (not tied to layers)
    pub delays: HashMap<u64, Delay>,

    /// Next animation ID generator
    next_animation_id: u64,

    /// Callback maps keyed by animation ID
    pub callbacks_on_start: HashMap<u64, Vec<Box<dyn FnOnce(&mut EmotivaHeart)>>>,
    pub callbacks_on_complete: HashMap<u64, Vec<Box<dyn FnOnce(&mut EmotivaHeart)>>>,
}

impl EmotivaHeart {
    pub fn new(rig: CharRig) -> Self {
        let mut tweens = HashMap::new();
        let mut motions = HashMap::new();
        let mut rotations = HashMap::new();

        // Eyes
        let has_eyes = rig.layers.iter().any(|l| l.name.contains("eyes"));
        let eyes = if has_eyes {
            if let Some(cfg) = &rig.eyes {
                Some(EyesState::with_config(
                    cfg.blink_duration,
                    cfg.blink_interval_range,
                ))
            } else {
                Some(EyesState::new())
            }
        } else {
            None
        };

        // Mouth
        let has_mouth = rig.layers.iter().any(|l| l.name.contains("mouth"));
        let mouth = if has_mouth {
            if let Some(cfg) = &rig.mouth {
                Some(MouthState::with_config(
                    cfg.talk_duration,
                    cfg.talk_interval,
                    cfg.flap_time,
                ))
            } else {
                Some(MouthState::new())
            }
        } else {
            None
        };

        // Layers
        for layer in rig.layers.iter() {
            if layer.tween.is_some() {
                tweens.insert(layer.name.clone(), TweenState::new());
            }

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
            delays: HashMap::new(),
            next_animation_id: 1,
            callbacks_on_start: HashMap::new(),
            callbacks_on_complete: HashMap::new(),
        }
    }

    /// Advance animation state by delta time (in seconds)
    pub fn update(&mut self, delta_time: f32, rng: &mut impl Rng) {
        self.time += delta_time;

        // collect events from all systems
        let mut events: Vec<AnimEvent> = Vec::new();

        if let Some(eye) = &mut self.eyes {
            let e = eye.update(self.time, rng);
            events.push(e);
        }

        if let Some(mouth) = &mut self.mouth {
            let e = mouth.update(self.time, rng);
            events.push(e);
        }

        for tween in self.tweens.values_mut() {
            let e = tween.update(delta_time);
            events.push(e);
        }

        for motion in self.motions.values_mut() {
            let e = motion.update(delta_time);
            events.push(e);
        }

        for rotation in self.rotations.values_mut() {
            let e = rotation.update(delta_time);
            events.push(e);
        }

        let e = self.visual_fx.update(delta_time);
        events.push(e);

        // tick delays
        let mut finished_ids = Vec::new();
        for (_id, delay) in self.delays.iter_mut() {
            let ev = delay.update(delta_time);
            events.push(ev);
            if let AnimEvent::Completed(Some(done_id)) = ev {
                finished_ids.push(done_id);
            }
        }
        for id in finished_ids {
            self.delays.remove(&id);
        }

        // handle events and fire callbacks
        for event in events {
            self.handle_event(event);
        }
    }

    /// Returns transformed sprites to render this frame.
    pub fn get_drawables(&mut self) -> Vec<DrawableSprite> {
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

            let fallback = &WorldTransform::default();
            let transform = transforms.get(&layer.name).unwrap_or(fallback);

            let final_image = self
                .image_overrides
                .get(&layer.name)
                .cloned()
                .unwrap_or_else(|| layer.image.clone());

            output.push(DrawableSprite {
                image: final_image,
                position: (transform.position.0, transform.position.1),
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

    // ================================ ID System ==============================//
    /// Generate and return the next unique animation ID.
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_animation_id;
        self.next_animation_id += 1;
        id
    }

    // ==================== Delay API ==================== //
    pub fn set_delay(&mut self, duration: f32) -> u64 {
        let id = self.next_id();
        let mut delay = Delay::new(duration);
        delay.set_animation_id(id);
        delay.play();
        self.delays.insert(id, delay);
        id
    }

    pub fn on_delay<F>(&mut self, duration: f32, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        let id = self.set_delay(duration);
        self.register_callback_on_complete(id, cb);
    }

    // ================================ Callbacks API =================================//
    pub fn register_callback_on_start<F>(&mut self, id: u64, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        self.callbacks_on_start
            .entry(id)
            .or_default()
            .push(Box::new(cb));
    }

    pub fn register_callback_on_complete<F>(&mut self, id: u64, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        self.callbacks_on_complete
            .entry(id)
            .or_default()
            .push(Box::new(cb));
    }

    fn handle_event(&mut self, event: AnimEvent) {
        match event {
            AnimEvent::Started(Some(id)) => {
                if let Some(cbs) = self.callbacks_on_start.remove(&id) {
                    for cb in cbs {
                        cb(self);
                    }
                }
            }
            AnimEvent::Completed(Some(id)) => {
                if let Some(cbs) = self.callbacks_on_complete.remove(&id) {
                    for cb in cbs {
                        cb(self);
                    }
                }
            }
            _ => {}
        }
    }
}
