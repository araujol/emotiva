/// Serializable snapshot of the **runtime animation state** of `Emotiva`.
///
/// This snapshot represents a *pause-the-universe* capture of Emotiva’s internal
/// state. It is intentionally limited to **pure, deterministic runtime data**:
///
/// - No asset definitions (`CharRig`)
/// - No rendering artifacts (`DrawableSprite`, textures)
/// - No RNG state
/// - No callbacks or script-side logic
///
/// Restoring a snapshot guarantees that the next `update(dt)` will continue
/// animation exactly where it left off, as if time had been frozen.
///
/// ⚠️ Important semantics:
/// - `time` is a monotonic, internal clock used by procedural systems
///   (eyes, mouth, FX, delays). It is preserved to maintain behavioral continuity.
/// - Callbacks are *not* restored on load. They are considered script-time effects
///   and are intentionally dropped to avoid duplicated or out-of-context execution.
///
/// This design allows save/load to behave like a true suspension of the animation
/// system rather than a visual reconstruction.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Emotiva;
use crate::anim::eyes::EyesState;
use crate::anim::mouth::MouthState;
use crate::core::delay::Delay;
use crate::core::fx::VisualFxState;
use crate::core::motion::{Motion2D, Rotation};
use crate::core::tween::TweenState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotivaSnapshot {
    pub time: f32,

    pub mouth: Option<MouthState>,
    pub eyes: Option<EyesState>,

    pub image_overrides: HashMap<String, String>,

    pub tweens: HashMap<String, TweenState>,
    pub motions: HashMap<String, Motion2D>,
    pub rotations: HashMap<String, Rotation>,

    pub visual_fx: VisualFxState,
    pub delays: HashMap<u64, Delay>,

    pub next_animation_id: u64,

    pub screen_position: (f32, f32),
}

impl Emotiva {
    /// Capture the current runtime animation state.
    ///
    /// The returned snapshot is sufficient to resume animation deterministically
    /// after a load, without reconstructing or replaying any animation logic.
    pub fn save_snapshot(&self) -> EmotivaSnapshot {
        EmotivaSnapshot {
            time: self.heart.time,
            mouth: self.heart.mouth.clone(),
            eyes: self.heart.eyes.clone(),
            image_overrides: self.heart.image_overrides.clone(),
            tweens: self.heart.tweens.clone(),
            motions: self.heart.motions.clone(),
            rotations: self.heart.rotations.clone(),
            visual_fx: self.heart.visual_fx.clone(),
            delays: self.heart.delays.clone(),
            next_animation_id: self.heart.next_animation_id,
            screen_position: self.screen_position,
        }
    }

    /// Restore a previously captured runtime animation state.
    ///
    /// Callbacks are intentionally cleared during restore, as they represent
    /// script-time side effects rather than persistent animation state.
    pub fn load_snapshot(&mut self, snap: EmotivaSnapshot) {
        self.heart.time = snap.time;
        self.heart.mouth = snap.mouth;
        self.heart.eyes = snap.eyes;
        self.heart.image_overrides = snap.image_overrides;
        self.heart.tweens = snap.tweens;
        self.heart.motions = snap.motions;
        self.heart.rotations = snap.rotations;
        self.heart.visual_fx = snap.visual_fx;
        self.heart.delays = snap.delays;
        self.heart.next_animation_id = snap.next_animation_id;
        self.screen_position = snap.screen_position;

        // Important: callbacks are intentionally cleared
        self.heart.callbacks_on_start.clear();
        self.heart.callbacks_on_complete.clear();
    }
}
