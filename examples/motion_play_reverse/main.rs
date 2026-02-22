// ==========================================
// 🎭 Emotiva Motion Play / Reverse Demo (examples/)
// ------------------------------------------
// This example demonstrates how to use Emotiva’s motion system
// to play and reverse a predefined movement animation at runtime.
//
// A character rig with a vertical motion track ("doll") is loaded,
// and the motion is toggled using keyboard input. The example shows
// how to control motion playback, detect when it finishes, and
// coordinate simple state transitions.
//
// 🌀 What it shows:
//  - How to trigger motion playback with `motion_play`
//  - How to reverse a motion using `motion_reverse`
//  - How to detect completion with `is_motion_finished`
//  - How to gate input while an animation is running
//  - How to manage simple animation state externally
//
// 🎮 Interaction:
//  - Press ENTER to toggle the character:
//      • Off-screen → center (play motion)
//      • Center → off-screen (reverse motion)
//
// This example highlights how Emotiva’s motion tracks can be
// controlled imperatively and integrated with real-time input.
//
// ▶️ Run this example with:
// ```bash
// cargo run --example motion_play_reverse
// ```
// ==========================================
use emotiva::Emotiva;
use emotiva::api::EmotivaAPI;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Emotiva - Up/Down Motion Example".to_string(),
        window_width: 1536,
        window_height: 1024,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let background = load_texture("examples/assets/dojo.png").await.unwrap();

    let mut emotiva = Emotiva::load_with_textures(
        "examples/motion_play_reverse/motion_play_reverse_doll.emotiva.ron",
        "examples/assets/",
    )
    .await;

    emotiva.set_screen_position((768.0, 1024.0));

    let mut is_centered = false;
    let mut is_animating = false;

    loop {
        let dt = get_frame_time();
        clear_background(GRAY);

        // ───── Input ─────
        if is_key_pressed(KeyCode::Enter) && !is_animating {
            is_animating = true;

            if is_centered {
                // Center → off-screen
                emotiva.motion_reverse("doll");
            } else {
                // Off-screen → center
                emotiva.motion_play("doll");
            }
        }

        // ───── Animation finished? ─────
        if is_animating && emotiva.is_motion_finished("doll") {
            is_animating = false;
            is_centered = !is_centered;
        }

        emotiva.update(dt);
        draw_texture(&background, 0.0, 0.0, WHITE);
        // ───── UI Hint ─────
        draw_text("Press ENTER to toggle motion", 40.0, 60.0, 40.0, BLACK);
        emotiva.draw();

        next_frame().await;
    }
}
