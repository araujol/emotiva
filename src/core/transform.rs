//! transform.rs
//! Emotiva Transform – World transform resolution system
//!
//! This module resolves per-layer world transforms, supporting local offsets,
//! tween/motion animation, visual FX, and recursive parent-child hierarchy.
//!
//! Each frame, `resolve_all_transforms` calculates the final `WorldTransform`
//! for every layer in the rig, blending tweening, motion, rotation, scale,
//! alpha, and tint effects — all while applying inherited transformations from parents.
//!
//! Used internally by `CharAnimator` to produce accurate, time-based transforms
//! for rendering animated characters.

use std::collections::HashMap;

use crate::core::fx::VisualFxState;
use crate::core::motion::{Motion2D, Rotation};
use crate::core::tween::TweenState;
use crate::format::CharRig;

#[derive(Debug, Clone, Copy)]
pub struct WorldTransform {
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: f32,
    pub alpha: f32,
    pub tint: [f32; 4],
}

impl Default for WorldTransform {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            rotation: 0.0,
            scale: 1.0,
            alpha: 1.0,
            tint: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

/// Resolves world transforms for all layers, accounting for parenting and all transformation sources.
pub fn resolve_all_transforms(
    rig: &CharRig,
    tweens: &mut HashMap<String, TweenState>,
    motions: &HashMap<String, Motion2D>,
    rotations: &HashMap<String, Rotation>,
    fx: &VisualFxState,
) -> HashMap<String, WorldTransform> {
    let mut result = HashMap::new();

    for layer in &rig.layers {
        let transform = resolve_layer_transform(
            &layer.name,
            rig,
            &mut result,
            tweens,
            motions,
            rotations,
            fx,
        );
        result.insert(layer.name.clone(), transform);
    }

    result
}

fn resolve_layer_transform(
    name: &str,
    rig: &CharRig,
    cache: &mut HashMap<String, WorldTransform>,
    tweens: &mut HashMap<String, TweenState>,
    motions: &HashMap<String, Motion2D>,
    rotations: &HashMap<String, Rotation>,
    fx: &VisualFxState,
) -> WorldTransform {
    let layer = rig.layers.iter().find(|l| l.name == name).unwrap();

    // Base local position
    let mut pos = layer.offset.unwrap_or((0.0, 0.0));
    let mut rot = 0.0;
    let mut scale = layer.scale.unwrap_or(1.0);
    let mut alpha = 1.0;
    let mut tint = [1.0, 1.0, 1.0, 1.0];

    // Tween
    if let Some(tween_def) = &layer.tween {
        if let Some(state) = tweens.get_mut(&layer.name) {
            let offs = state.value(tween_def);
            pos.0 += offs.dx;
            pos.1 += offs.dy;
            rot += offs.rotation;
        }
    }

    // Motion
    if let Some(m) = motions.get(&layer.name) {
        let (dx, dy) = m.value();
        pos.0 += dx;
        pos.1 += dy;
    }

    // Rotation
    if let Some(r) = rotations.get(&layer.name) {
        rot += r.value();
    }

    // FX
    if let Some(fx_offset) = fx.get_fx(&layer.name) {
        if let Some(s) = fx_offset.scale {
            scale *= s;
        }
        if let Some(a) = fx_offset.alpha {
            alpha *= a;
        }
        if let Some(t) = fx_offset.tint {
            tint = t;
        }
    }

    // Build local transform
    let local = WorldTransform {
        position: pos,
        rotation: rot,
        scale,
        alpha,
        tint,
    };

    // Parenting
    if let Some(parent_name) = &layer.parent {
        if layer.inherit.unwrap_or(true) {
            let fallback = WorldTransform::default();
            let parent = cache.get(parent_name).unwrap_or(&fallback);
            return combine(parent, &local);
        }
    }

    local
}

fn combine(parent: &WorldTransform, local: &WorldTransform) -> WorldTransform {
    let rotated_offset = rotate_point(local.position, parent.rotation);
    WorldTransform {
        position: (
            parent.position.0 + rotated_offset.0,
            parent.position.1 + rotated_offset.1,
        ),
        rotation: parent.rotation + local.rotation,
        scale: parent.scale * local.scale,
        alpha: parent.alpha * local.alpha,
        tint: local.tint, // tint is not inherited
    }
}

fn rotate_point(point: (f32, f32), angle: f32) -> (f32, f32) {
    let cos = angle.cos();
    let sin = angle.sin();
    (point.0 * cos - point.1 * sin, point.0 * sin + point.1 * cos)
}
