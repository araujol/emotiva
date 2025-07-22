//! fx.rs
//! Emotiva FX – Visual-Only Effects Module
//!
//! This module applies visual-only effects such as scale and alpha animations
//! (e.g., scale up/down, fade in/out) independently from position or expression logic.
//! These effects are applied per-layer via API only (not loaded from .ron files).
//! Designed for composability and integration with `CharAnimator`.

use crate::easing::{Easing, resolve as resolve_easing};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ScaleFxKind {
    pub from: f32,
    pub to: f32,
    pub duration: f32,
    pub easing: Easing,
    pub elapsed: f32,
    pub finished: bool,
}

#[derive(Clone, Debug)]
pub struct AlphaFxKind {
    pub from: f32,
    pub to: f32,
    pub duration: f32,
    pub easing: Easing,
    pub elapsed: f32,
    pub finished: bool,
}

#[derive(Clone, Debug, Default)]
pub struct TransformOffset {
    pub scale: Option<f32>,
    pub alpha: Option<f32>,
}

#[derive(Debug)]
pub struct VisualFxState {
    time: f32,
    scale_fx: HashMap<String, ScaleFxKind>,
    alpha_fx: HashMap<String, AlphaFxKind>,
    pub base_scale: HashMap<String, f32>,
}

impl VisualFxState {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            scale_fx: HashMap::new(),
            alpha_fx: HashMap::new(),
            base_scale: HashMap::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        for fx in self.scale_fx.values_mut() {
            if !fx.finished {
                fx.elapsed += dt;
                if fx.elapsed >= fx.duration {
                    fx.elapsed = fx.duration;
                    fx.finished = true;
                }
            }
        }

        for fx in self.alpha_fx.values_mut() {
            if !fx.finished {
                fx.elapsed += dt;
                if fx.elapsed >= fx.duration {
                    fx.elapsed = fx.duration;
                    fx.finished = true;
                }
            }
        }
    }

    pub fn get_fx(&self, layer: &str) -> Option<TransformOffset> {
        let mut result = TransformOffset::default();

        if let Some(fx) = self.scale_fx.get(layer) {
            let progress = (fx.elapsed / fx.duration).min(1.0);
            let factor = resolve_easing(fx.easing, progress);
            result.scale = Some(fx.from + (fx.to - fx.from) * factor);
        }

        if let Some(fx) = self.alpha_fx.get(layer) {
            let progress = (fx.elapsed / fx.duration).min(1.0);
            let factor = resolve_easing(fx.easing, progress);
            result.alpha = Some(fx.from + (fx.to - fx.from) * factor);
        }

        if result.scale.is_some() || result.alpha.is_some() {
            Some(result)
        } else {
            None
        }
    }

    // Methods intended to be used by API
    pub fn add_scale_fx(&mut self, layer: &str, fx: ScaleFxKind) {
        self.scale_fx.insert(layer.to_string(), fx);
    }

    pub fn add_alpha_fx(&mut self, layer: &str, fx: AlphaFxKind) {
        self.alpha_fx.insert(layer.to_string(), fx);
    }

    pub fn remove_scale_fx(&mut self, layer: &str) {
        self.scale_fx.remove(layer);
    }

    pub fn remove_alpha_fx(&mut self, layer: &str) {
        self.alpha_fx.remove(layer);
    }

    pub fn clear_all_fx(&mut self) {
        self.scale_fx.clear();
        self.alpha_fx.clear();
    }
}

// === FX API Helpers ===
/// Returns a scale animation from one value to another
pub fn make_scale_fx(from: f32, to: f32, duration: f32, easing: Easing) -> ScaleFxKind {
    ScaleFxKind {
        from,
        to,
        duration,
        easing,
        elapsed: 0.0,
        finished: false,
    }
}

/// Returns an alpha animation from one value to another
pub fn make_alpha_fx(from: f32, to: f32, duration: f32, easing: Easing) -> AlphaFxKind {
    AlphaFxKind {
        from,
        to,
        duration,
        easing,
        elapsed: 0.0,
        finished: false,
    }
}
