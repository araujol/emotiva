// ==========================================
// 🎛 Emotiva Tween Lifecycle Control Demo (examples/)
// ------------------------------------------
// This example demonstrates how to control a layer tween at runtime,
// including starting, pausing, resuming, and stopping animations.
//
// A rig with a predefined tween ("doll") is loaded, and the example
// automatically cycles through different tween states over time.
// The goal is to showcase how Emotiva allows fine-grained control
// over continuous layer animations without restarting the engine
// or rebuilding the rig.
//
// 🌀 What it shows:
//  - How to start a tween with `tween_start`
//  - How to pause and resume a tween
//  - How to fully stop a tween
//  - How to query tween state (`is_tween_paused`)
//  - How to coordinate tween control using simple runtime logic
//
// 🕒 Runtime Behavior:
//  - Every ~5 seconds the tween state changes
//  - The cycle alternates between running and paused states
//  - After several pauses, the tween is fully stopped
//  - The sequence then repeats
//
// This example highlights how Emotiva supports dynamic animation
// control during gameplay or interactive scenes.
//
// ▶️ Run this example with:
// ```bash
// cargo run --example tween_lifecycle
// ```
// ==========================================
use emotiva::Emotiva;
use emotiva::api::EmotivaAPI;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Emotiva - Tween Lifecycle".to_string(),
        window_width: 1536,
        window_height: 1024,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture: Texture2D = load_texture("examples/assets/dojo.png").await.unwrap();

    let mut emotiva = Emotiva::load_with_textures(
        "examples/tween_lifecycle/tween_lifecycle_doll.emotiva.ron",
        "examples/assets/",
    )
    .await;

    let mut elapsed = 0.0;
    let mut toggled = true;
    let mut i = 0;

    loop {
        clear_background(GRAY);

        let delta = get_frame_time();

        elapsed += delta;
        if elapsed >= 5.0 {
            if toggled {
                if emotiva.is_tween_paused("doll") {
                    emotiva.tween_resume("doll");
                } else {
                    emotiva.tween_start("doll");
                }
            } else {
                if i >= 3 {
                    i = 0;
                    emotiva.tween_stop("doll");
                } else {
                    emotiva.tween_pause("doll");
                }
            }

            toggled = !toggled;
            elapsed = 0.0;
            i += 1;
        }

        emotiva.update(delta);
        draw_dojo(texture.clone());
        // ───── UI Hint ─────
        draw_text(
            "Wait for horizontal movement to start",
            40.0,
            60.0,
            40.0,
            BLACK,
        );
        emotiva.draw();

        next_frame().await;
    }
}

fn draw_dojo(texture: Texture2D) {
    draw_texture_ex(
        &texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(texture.width(), texture.height())),
            ..Default::default()
        },
    );
}
