//! # Emotiva Motion Play / Reverse Example
//!
//! Demonstrates how to control **motion animations** in Emotiva by
//! playing and reversing a predefined motion track at runtime.
//!
//! The example loads a character rig with a vertical motion track
//! (`doll`) and toggles the animation using keyboard input.
//!
//! ## What This Example Shows
//!
//! - Triggering a motion using `motion_play`
//! - Reversing a motion using `motion_reverse`
//! - Checking animation state with `is_motion_playing`
//! - Preventing input while an animation is running
//! - Managing simple animation state externally
//!
//! ## Interaction
//!
//! Press **ENTER** to toggle the character motion:
//!
//! - Off-screen → center (`motion_play`)
//! - Center → off-screen (`motion_reverse`)
//!
//! ## Run
//!
//! ```bash
//! cargo run --example motion_play_reverse
//! ```
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

    loop {
        let dt = get_frame_time();
        clear_background(GRAY);

        // ───── Input ─────
        if is_key_pressed(KeyCode::Enter) && !emotiva.is_motion_playing("doll") {
            if is_centered {
                // Center → off-screen
                emotiva.motion_reverse("doll");
                is_centered = false;
            } else {
                // Off-screen → center
                emotiva.motion_play("doll");
                is_centered = true;
            }
        }

        emotiva.update(dt);
        draw_texture(&background, 0.0, 0.0, WHITE);
        // ───── UI Hint ─────
        draw_text("Press ENTER to toggle motion", 40.0, 60.0, 40.0, BLACK);
        emotiva.draw();

        next_frame().await;
    }
}
