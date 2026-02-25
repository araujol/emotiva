# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
- Placeholder for upcoming changes.

## [0.1.0] - 2026-02-25

### Added
- Core animation runtime with layered sprite composition.
- `EmotivaRig` structure for rig definition and animation configuration.
- Tween system with:
  - Easing support.
  - Pause/resume control.
  - Start/stop logic.
  - Decoupled state update and value calculation.
- Motion system with:
  - Offset-based layer animation.
  - Rotation-based motion via dedicated `Rotation` type.
  - Sway tweening support.
  - Layer parenting and hierarchical transforms.
- Eyes animation module with randomized blinking and session-based control.
- Mouth talking API with session-based flap rhythm and event control.
- Visual FX system supporting:
  - Scale and alpha animations.
  - Color tint effects.
- Unique animation ID system for runtime control and tracking.
- Animation callback system with `on_start` and `on_end` helpers.
- `AnimEvent` system shared across motion, tween, and FX modules.
- Delay animation module with unified callback handling.
- Snapshot-based save/load support for runtime animation state.
- `EmotivaAPI` trait for unified frontend/backend abstraction.
- `EmotivaHeart` core engine structure.
- WASM-safe architecture with async rig loading and RNG abstraction.
- Structured feature-based example demos.

### Changed
- Renamed `CharRig` to `EmotivaRig` for clearer domain alignment.
- Renamed `DrawableSprite` to `EmotivaForm` and updated API to use `forms()`.
- Replaced `is_finished` with `is_playing` for clearer animation state semantics.
- Decoupled core runtime from Macroquad frontend.
- Refactored ID system for unified animation tracking.
- Standardized tween API and routed easing configuration through rig files.
- Updated internal tween storage to `HashMap<String, TweenState>`.
- Improved API ergonomics by using `&self` where mutation is not required.
- Renamed eyes-related API methods and parameters for clarity.

### Fixed
- Tweens now initialize only if defined in rig files.
- Corrected example integrations after API refactors.

### Removed
- Tight coupling between runtime core and Macroquad-specific logic.