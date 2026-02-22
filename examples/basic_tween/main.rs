// ==========================================
// 🎭 Emotiva Character Basic Tweening Demo (examples/)
// ------------------------------------------
// This example demonstrates how to initialize and run a basic
// character rig using Emotiva with idle tween animations.
//
// The demo loads Kaori’s rig definition, applies automatic
// blinking and layered motion tweens (body sway, hair movement),
// and renders the character over a static background using
// Macroquad.
//
// 🌀 What it shows:
//  - How to load a `.emotiva.ron` rig file
//  - How to provide a texture directory for sprite layers
//  - How to position a character on screen
//  - How to start tweens programmatically
//  - How to update and draw Emotiva each frame
//
// 🕒 Runtime Behavior:
//  - On first frame, blinking and body/hair tweens are started
//  - The character animates continuously using idle motion
//  - Background is rendered first, then the animated character
//
// ▶️ Run this example with:
// ```bash
// cargo run --example kaori
// ```
// ==========================================
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
