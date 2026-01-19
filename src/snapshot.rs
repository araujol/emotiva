/// Serializable snapshot of the **runtime animation state** of an `EmotivaHeart`.
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

use crate::EmotivaHeart;
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
}

impl EmotivaHeart {
    /// Capture the current runtime animation state.
    ///
    /// The returned snapshot is sufficient to resume animation deterministically
    /// after a load, without reconstructing or replaying any animation logic.
    pub fn save_snapshot(&self) -> EmotivaSnapshot {
        EmotivaSnapshot {
            time: self.time,
            mouth: self.mouth.clone(),
            eyes: self.eyes.clone(),
            image_overrides: self.image_overrides.clone(),
            tweens: self.tweens.clone(),
            motions: self.motions.clone(),
            rotations: self.rotations.clone(),
            visual_fx: self.visual_fx.clone(),
            delays: self.delays.clone(),
            next_animation_id: self.next_animation_id,
        }
    }

    /// Restore a previously captured runtime animation state.
    ///
    /// Callbacks are intentionally cleared during restore, as they represent
    /// script-time side effects rather than persistent animation state.
    pub fn load_snapshot(&mut self, snap: EmotivaSnapshot) {
        self.time = snap.time;
        self.mouth = snap.mouth;
        self.eyes = snap.eyes;
        self.image_overrides = snap.image_overrides;
        self.tweens = snap.tweens;
        self.motions = snap.motions;
        self.rotations = snap.rotations;
        self.visual_fx = snap.visual_fx;
        self.delays = snap.delays;
        self.next_animation_id = snap.next_animation_id;

        // Important: callbacks are intentionally cleared
        self.callbacks_on_start.clear();
        self.callbacks_on_complete.clear();
    }
}
