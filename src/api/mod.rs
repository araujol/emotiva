// ==========================================
// 🎭 Emotiva API Module (mod.rs)
// ------------------------------------------
// This is the **main API entry point** for Emotiva.
//
// ✅ Responsibilities:
//  - Organize and re-export all sub-API modules (tween, motion, fx, anim)
//  - Serve as a single import point for all public-facing Emotiva functions
//  - Provide a clear structure to keep lib.rs clean and maintainable
//
// 📦 Usage:
// Frontend projects only need to import from this module to access the
// full Emotiva API surface:
// ```rust
// use emotiva::api::*;
// ```
// ==========================================

pub mod anim;
pub mod callback;
pub mod fx;
pub mod layer;
pub mod motion;
pub mod tween;
