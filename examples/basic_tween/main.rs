//! # Emotiva Basic Tweening Example
//!
//! Demonstrates how to load and animate a character rig using **Emotiva**
//! with idle tween animations and blinking behavior.
//!
//! The example loads Kaori's rig, starts blinking and several layer tweens,
//! and renders the animated character over a static background using
//! the Macroquad backend.
//!
//! ## What This Example Shows
//!
//! - Loading a `.emotiva.ron` rig file
//! - Providing a texture directory for sprite layers
//! - Positioning a character on screen
//! - Starting tween animations programmatically
//! - Updating and rendering Emotiva each frame
//!
//! ## Runtime Behavior
//!
//! On the first frame the example:
//!
//! - Enables automatic eye blinking
//! - Starts idle tweens for body and hair layers
//!
//! The character then animates continuously using these motions.
//!
//! ## Run
//!
//! ```bash
//! cargo run --example basic_tween
//! ```
use emotiva::Emotiva;
use emotiva::api::EmotivaAPI;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Emotiva - Basic Character Tweening".to_string(),
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
        "examples/basic_tween/kaori.emotiva.ron",
        "examples/assets/kaori",
    )
    .await;
    emotiva.set_screen_position((768.0, 512.0));

    loop {
        clear_background(GRAY);

        let delta = get_frame_time();

        if !emotiva.is_tween_enabled("body") {
            emotiva.eyes_start();
            emotiva.tween_start("body");
            emotiva.tween_start("hair_behind");
            emotiva.tween_start("hair_front");
        }

        emotiva.update(delta);
        draw_dojo(texture.clone());
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
