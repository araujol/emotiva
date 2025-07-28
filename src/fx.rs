//! fx.rs
//! Emotiva FX – Visual-Only Effects Module
//!
//! This module applies visual-only effects such as scale and alpha animations
//! (e.g., scale up/down, fade in/out) independently from position or expression logic.
//! These effects are applied per-layer via API only (not loaded from .ron files).
//! Designed for composability and integration with `CharAnimator`.

use crate::easing::{Easing, resolve as resolve_easing};
use crate::events::AnimEvent;
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

#[derive(Clone, Debug)]
pub struct TintFxKind {
    pub from: [f32; 4],
    pub to: [f32; 4],
    pub duration: f32,
    pub easing: Easing,
    pub elapsed: f32,
    pub finished: bool,
}

#[derive(Clone, Debug, Default)]
pub struct TransformOffset {
    pub scale: Option<f32>,
    pub alpha: Option<f32>,
    pub tint: Option<[f32; 4]>,
}

#[derive(Debug)]
pub struct VisualFxState {
    time: f32,
    scale_fx: HashMap<String, ScaleFxKind>,
    alpha_fx: HashMap<String, AlphaFxKind>,
    tint_fx: HashMap<String, TintFxKind>,
}

impl VisualFxState {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            scale_fx: HashMap::new(),
            alpha_fx: HashMap::new(),
            tint_fx: HashMap::new(),
        }
    }

    /// Updates all FX objects and reports if any started or finished.
    pub fn update(&mut self, dt: f32) -> AnimEvent {
        self.time += dt;
        let mut event = AnimEvent::None;

        fn advance(elapsed: &mut f32, duration: f32, finished: &mut bool, dt: f32) -> AnimEvent {
            if !*finished {
                *elapsed += dt;
                if *elapsed >= duration {
                    *elapsed = duration;
                    *finished = true;
                    return AnimEvent::Completed;
                }
                if *elapsed == 0.0 {
                    return AnimEvent::Started;
                }
            }
            AnimEvent::None
        }

        // FIX: Emitted signals should differentiate FX.
        for fx in self.scale_fx.values_mut() {
            let e = advance(&mut fx.elapsed, fx.duration, &mut fx.finished, dt);
            if let AnimEvent::Started | AnimEvent::Completed = e {
                event = e;
            }
        }

        for fx in self.alpha_fx.values_mut() {
            let e = advance(&mut fx.elapsed, fx.duration, &mut fx.finished, dt);
            if let AnimEvent::Started | AnimEvent::Completed = e {
                event = e;
            }
        }

        for fx in self.tint_fx.values_mut() {
            let e = advance(&mut fx.elapsed, fx.duration, &mut fx.finished, dt);
            if let AnimEvent::Started | AnimEvent::Completed = e {
                event = e;
            }
        }

        event
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

        if let Some(fx) = self.tint_fx.get(layer) {
            let progress = (fx.elapsed / fx.duration).min(1.0);
            let factor = resolve_easing(fx.easing, progress);
            let tint = [
                fx.from[0] + (fx.to[0] - fx.from[0]) * factor,
                fx.from[1] + (fx.to[1] - fx.from[1]) * factor,
                fx.from[2] + (fx.to[2] - fx.from[2]) * factor,
                fx.from[3] + (fx.to[3] - fx.from[3]) * factor,
            ];
            result.tint = Some(tint);
        }

        if result.scale.is_some() || result.alpha.is_some() || result.tint.is_some() {
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

    pub fn add_tint_fx(&mut self, layer: &str, fx: TintFxKind) {
        self.tint_fx.insert(layer.to_string(), fx);
    }

    pub fn remove_scale_fx(&mut self, layer: &str) {
        self.scale_fx.remove(layer);
    }

    pub fn remove_alpha_fx(&mut self, layer: &str) {
        self.alpha_fx.remove(layer);
    }

    pub fn remove_tint_fx(&mut self, layer: &str) {
        self.tint_fx.remove(layer);
    }

    pub fn clear_all_fx(&mut self) {
        self.scale_fx.clear();
        self.alpha_fx.clear();
        self.tint_fx.clear();
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

/// Returns a tint animation from one color to another (RGBA)
pub fn make_tint_fx(from: [f32; 4], to: [f32; 4], duration: f32, easing: Easing) -> TintFxKind {
    TintFxKind {
        from,
        to,
        duration,
        easing,
        elapsed: 0.0,
        finished: false,
    }
}
