//! core/fx.rs
//! Emotiva FX – Visual-Only Effects Module
//!
//! This module applies visual-only effects such as scale and alpha animations
//! (e.g., scale up/down, fade in/out) independently from position or expression logic.
//! These effects are applied per-layer via API only (not loaded from .ron files).
//! Designed for composability and integration with `CharAnimator`.

use crate::core::easing::{Easing, resolve as resolve_easing};
use crate::core::events::AnimEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScaleFxKind {
    pub from: f32,
    pub to: f32,
    pub duration: f32,
    pub easing: Easing,
    pub elapsed: f32,
    pub finished: bool,
    pub animation_id: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlphaFxKind {
    pub from: f32,
    pub to: f32,
    pub duration: f32,
    pub easing: Easing,
    pub elapsed: f32,
    pub finished: bool,
    pub animation_id: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TintFxKind {
    pub from: [f32; 4],
    pub to: [f32; 4],
    pub duration: f32,
    pub easing: Easing,
    pub elapsed: f32,
    pub finished: bool,
    pub animation_id: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransformOffset {
    pub scale: Option<f32>,
    pub alpha: Option<f32>,
    pub tint: Option<[f32; 4]>,
}

impl Default for TransformOffset {
    fn default() -> Self {
        Self {
            scale: None,
            alpha: None,
            tint: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

    /// Updates all FX objects and reports if any started or finished with their IDs.
    pub fn update(&mut self, dt: f32) -> AnimEvent {
        self.time += dt;
        let mut event = AnimEvent::None;

        fn advance(
            elapsed: &mut f32,
            duration: f32,
            finished: &mut bool,
            dt: f32,
            id: Option<u64>,
        ) -> AnimEvent {
            if !*finished {
                // First frame logic
                if *elapsed == 0.0 {
                    *elapsed += dt;
                    if *elapsed >= duration {
                        *elapsed = duration;
                        *finished = true;
                        return AnimEvent::Completed(id);
                    }
                    return AnimEvent::Started(id);
                }

                *elapsed += dt;
                if *elapsed >= duration {
                    *elapsed = duration;
                    *finished = true;
                    return AnimEvent::Completed(id);
                }
            }
            AnimEvent::None
        }

        for fx in self.scale_fx.values_mut() {
            let e = advance(
                &mut fx.elapsed,
                fx.duration,
                &mut fx.finished,
                dt,
                fx.animation_id,
            );
            if let AnimEvent::Started(_) | AnimEvent::Completed(_) = e {
                event = e;
            }
        }

        for fx in self.alpha_fx.values_mut() {
            let e = advance(
                &mut fx.elapsed,
                fx.duration,
                &mut fx.finished,
                dt,
                fx.animation_id,
            );
            if let AnimEvent::Started(_) | AnimEvent::Completed(_) = e {
                event = e;
            }
        }

        for fx in self.tint_fx.values_mut() {
            let e = advance(
                &mut fx.elapsed,
                fx.duration,
                &mut fx.finished,
                dt,
                fx.animation_id,
            );
            if let AnimEvent::Started(_) | AnimEvent::Completed(_) = e {
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

    /// Returns true if the scale FX on this layer has finished, or if none exists.
    pub fn is_scale_finished(&self, layer: &str) -> bool {
        match self.scale_fx.get(layer) {
            Some(fx) => fx.finished,
            None => true,
        }
    }

    /// Returns true if the alpha FX on this layer has finished, or if none exists.
    pub fn is_alpha_finished(&self, layer: &str) -> bool {
        match self.alpha_fx.get(layer) {
            Some(fx) => fx.finished,
            None => true,
        }
    }

    /// Returns true if the tint FX on this layer has finished, or if none exists.
    pub fn is_tint_finished(&self, layer: &str) -> bool {
        match self.tint_fx.get(layer) {
            Some(fx) => fx.finished,
            None => true,
        }
    }
}

// === FX API Helpers ===

/// Returns a scale animation from one value to another
pub fn make_scale_fx(
    from: f32,
    to: f32,
    duration: f32,
    easing: Easing,
    animation_id: Option<u64>,
) -> ScaleFxKind {
    ScaleFxKind {
        from,
        to,
        duration,
        easing,
        elapsed: 0.0,
        finished: false,
        animation_id,
    }
}

/// Returns an alpha animation from one value to another
pub fn make_alpha_fx(
    from: f32,
    to: f32,
    duration: f32,
    easing: Easing,
    animation_id: Option<u64>,
) -> AlphaFxKind {
    AlphaFxKind {
        from,
        to,
        duration,
        easing,
        elapsed: 0.0,
        finished: false,
        animation_id,
    }
}

/// Returns a tint animation from one color to another (RGBA)
pub fn make_tint_fx(
    from: [f32; 4],
    to: [f32; 4],
    duration: f32,
    easing: Easing,
    animation_id: Option<u64>,
) -> TintFxKind {
    TintFxKind {
        from,
        to,
        duration,
        easing,
        elapsed: 0.0,
        finished: false,
        animation_id,
    }
}
