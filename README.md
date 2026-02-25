# 🎭 Emotiva

**Expressive 2D character animation for Rust.**

Emotiva is a layered 2D character animation runtime designed for visual novels and character-driven games.
It loads `.emotiva.ron` rigs and drives animation systems (tweens, motion tracks, blinking, etc.), producing fully resolved sprite data each frame.

Macroquad support is enabled by default for convenience, but it is entirely optional. Emotiva can be used purely as an animation engine to generate resolved sprite output for any rendering backend.

It is small.
It is focused.
It animates.

---

## ✨ Features

* Layered sprite rig system (`.emotiva.ron`)
* Idle tween animation (breathing, sway, hair motion)
* Named motion tracks (play, reverse, query state)
* Automatic blinking system
* Renderer-agnostic core architecture
* Macroquad integration enabled by default
* Native + WASM support
* Deterministic RNG on `wasm32`
* Hierarchical layer transform system (coordinated multi-part animation)

---

## 🧩 Hierarchical Layer Animation

Emotiva treats a character as a structured stack of sprite layers resolved through a coordinated transform pipeline.

This is not bone-based skinning or weighted mesh deformation. Instead, it is a hierarchical 2D layer system designed for clean, synchronized multi-part animation.

Each `Layer` participates in a shared transform resolution process:

* Position offsets are resolved hierarchically
* Tweens apply synchronized sway and lean motion
* Motion tracks move the full character while preserving layer alignment
* Rotation and easing are resolved consistently across all parts

The result is a cohesive multi-layer animation model where:

* Body, hair, eyes, and accessories move together
* Individual parts can animate independently
* All transforms remain visually synchronized

You define the structure declaratively in `.emotiva.ron`.
Emotiva resolves and animates it as a single coordinated character entity.

---

## 📦 Installation

Add Emotiva to your `Cargo.toml`:

```toml
[dependencies]
emotiva = "0.1.0"
```

### Feature Flags

Macroquad support is **enabled by default**.

If you want a pure runtime without rendering helpers:

```toml
[dependencies]
emotiva = { version = "0.1.0", default-features = false }
```

Available features:

```toml
[features]
default = ["macroquad"]
macroquad = ["dep:macroquad"]
```

---

# 🚀 Running the Examples

Emotiva includes multiple working examples inside the `examples/` directory.

To see available examples, execute from the top directory:

```bash
cd emotiva/
cargo run --example
```

To run a specific example:

```bash
cargo run --example <name>
```

Example source files are located in:

```
examples/
```

These demonstrate:

* Loading `.emotiva.ron` rigs
* Providing texture directories
* Starting tweens programmatically
* Playing and reversing motion tracks
* Querying animation state
* Integrating with real-time input
* Managing animation lifecycle

If you want to understand how Emotiva is intended to be used, start there.

---

# 🗂 Rig Format Overview

Character definitions are written in `.emotiva.ron` files.

A rig describes the structural composition and animation behavior of a character in a fully declarative way.

At a high level, a rig contains:

* A collection of layered sprites that form the visual structure of the character
* Optional hierarchical relationships between layers (parent / inherit)
* Optional tween definitions for idle motion (breathing, sway, lean)
* Optional motion tracks for directional or one-shot animation
* Optional eye animation configuration (automatic blinking)
* Optional mouth animation configuration (talking / flap cycles)

The rig format is intentionally data-driven. You define how the character is built and how it should behave, and the runtime resolves those definitions into synchronized animated output.

For a complete and precise definition of all available fields and structures, see `format.rs`.

# 🧠 Runtime Architecture

Emotiva’s runtime flows from declarative data to resolved visual output.

### Core Types

* `EmotivaRig` — Parsed rig definition loaded from `.emotiva.ron`
* `EmotivaHeart` — Internal animation engine that owns all runtime state
* `EmotivaForm` — Fully resolved visual output for a frame
* `Emotiva` — Public runtime API and integration layer

### From Rig to Frame

1. A `.emotiva.ron` file is parsed into an `EmotivaRig`.
2. An `EmotivaHeart` is created from that rig.
3. The heart initializes animation systems (tweens, motion tracks, eyes, mouth).
4. On each `update(delta_time)`, the heart advances all animation state.
5. Hierarchical transforms are resolved.
6. One or more `EmotivaForm`s are produced for rendering.
7. The `Emotiva` API exposes these forms and, with default features, renders them via Macroquad.

`EmotivaHeart` is responsible for all animation logic and transform computation and 
`Emotiva` acts as the user-facing object to the complete public API.

---

# 🎮 Animation Control

Emotiva supports imperative runtime control:

* `tween_start(name)`
* `is_tween_enabled(name)`
* `motion_play(name)`
* `motion_reverse(name)`
* `is_motion_playing(name)`
* `eyes_start()`

This allows integration with:

* Input systems
* State machines
* Dialogue engines
* Cutscene logic

You control when animation happens.
Emotiva handles how it moves.

---

# 🌍 WASM Support

On `wasm32`:

* Uses deterministic `ChaCha8Rng`
* Avoids OS entropy
* Compiles cleanly for WebAssembly targets

Designed with portability in mind.

---

# 🎯 Design Philosophy

Emotiva follows a simple rule:

**Keep the core minimal. Let the API grow outward.**

* No engine lock-in
* No mandatory editor tooling
* No global state
* No rendering assumptions inside the core

It is meant to embed inside larger systems, not replace them.

---

# 📁 Source Repository Structure

```
src/
├── anim/          # Animation systems (eyes, mouth, etc.)
├── api/           # Public API
├── core/          # Tweening, motion, easing, transforms
├── format.rs      # Rig definitions
├── lib.rs         # Crate root, runtime and re-exports
├── snapshot.rs    # Runtime state capture
└── emotiva.rs     # Runtime entry point
```

---

# 🤝 Intended Use Cases

* Visual novel engines
* Dialogue-heavy games
* Character-focused 2D projects
* Narrative WebAssembly apps
* Custom Rust game engines

If you can render textured sprites in your engine, you can use Emotiva.

---

# 📜 License

Emotiva is licensed under the MIT License.

See the [LICENSE](LICENSE) file for details.

---

**Emotiva**

Emotion through motion.
