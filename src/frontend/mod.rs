// ==========================================
// 🎮 Emotiva Frontend Module (frontend/mod.rs)
// ------------------------------------------
// This module contains **platform-specific front-end integrations** for Emotiva.
//
// ✅ Responsibilities:
//  - Connect Emotiva's backend-agnostic core to rendering/input frameworks.
//  - Provide concrete implementations of the Emotiva API for specific platforms.
//  - Keep frontend logic separate from core engine code for maintainability.
//
// 📦 Current Implementations:
//  - quad.rs → Macroquad frontend for Emotiva.
//    Handles rendering sprites, updating animations, and integrating input
//    using the Macroquad game framework.
//
// 🔮 Future Expansion:
//  - Other frontends (Bevy, SDL2, etc.) can be added here as separate modules.
//
// This module is **optional** and feature-gated; it is included only when the
// corresponding frontend feature is enabled in Cargo.toml.
// ==========================================

pub mod quad;
