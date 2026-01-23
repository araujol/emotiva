// ==========================================
// 🧪 Emotiva Nested Callback Demo (examples/)
// ------------------------------------------
// This example demonstrates how to use nested animation callbacks
// in Emotiva to build a chained sequence of facial expressions and
// sprite effects using Sayuri's rig. The flow uses the `on_end`
// and `on_delay` APIs to layer animation stages step by step.
//
// 🌀 What it shows:
//  - How to chain animation callbacks
//  - How to build stateful animation loops with idle delays
//  - How to separate animation stages into clean helper functions
//
// 🕒 Animation Flow:
//  - Waits ~3 seconds at start
//  - Fades out and changes layers
//  - Pulses scale and activates motion
//  - Begins idle facial tweens (eyes, mouth, hair)
//  - Runs for ~20s, then stops
//  - Change animation settings at first stop
//  - And restarts after 3s , repeat loop
//
// ▶️ Run this example with:
// ```bash
// cargo run --example sayuri
// ```
// ==========================================

use emotiva::Emotiva;
use emotiva::EmotivaHeart;
use emotiva::api::Easing;
use emotiva::api::EmotivaAPI;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Testing Dojo".to_string(),
        window_width: 1536,
        window_height: 1024,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture: Texture2D = load_texture("test_data/dojo.png").await.unwrap();
    let mut emotiva =
        Emotiva::load_with_textures("test_data/sayuri.emotiva.ron", "test_sprites/sayuri").await;

    let mut elapsed = 0.0;
    let mut state = 0; // 0: Idle delay, 1: Running, 2: Cooldown

    loop {
        clear_background(WHITE);
        let delta = get_frame_time().min(0.1);
        elapsed += delta;

        match state {
            0 => {
                // Wait 3 seconds before first start
                if elapsed >= 3.0 && !emotiva.is_tween_enabled("base") {
                    start_intro_sequence(&mut emotiva);
                    state = 1; // switch to running
                    elapsed = 0.0;
                }
            }
            1 => {
                // Running: stop after 20 seconds
                if elapsed >= 20.0 {
                    stop_idle_cycle(&mut emotiva);
                    state = 0; // switch to running again
                    elapsed = 0.0;
                }
            }
            _ => {}
        }

        emotiva.update(delta);
        draw_dojo(texture.clone());
        emotiva.draw();

        next_frame().await;
    }
}

/// Starts a sequence of introductory animations for Sayuri.
/// This fades her out, changes some facial layers, then fades her back in.
fn start_intro_sequence(emotiva: &mut Emotiva) {
    let id = emotiva.set_alpha("base", 1.0, 0.0, 0.5, Easing::SineIn);
    emotiva.on_end(id, |emo| {
        intro_stage_two(emo);
    });
}

/// Second part of the intro animation.
/// Sets eyes/mouth layers, fades back in, applies scale pulse with motion.
fn intro_stage_two(emo: &mut EmotivaHeart) {
    emo.set_layer("eyes_open", "delighted");
    emo.set_layer("mouth_closed", "nothing");

    let id = emo.set_alpha("base", 0.0, 1.0, 0.5, Easing::SineInOut);
    emo.on_end(id, |emo| {
        let id = emo.set_scale("base", 1.0, 1.2, 0.8, Easing::Linear);
        emo.motion_play("base");
        emo.on_end(id, |emo| {
            intro_stage_three(emo);
        });
    });
}

/// Final part of the intro animation.
/// Shrinks back the scale, reverses motion, delays and resets layers.
fn intro_stage_three(emo: &mut EmotivaHeart) {
    let id = emo.set_scale("base", 1.2, 1.0, 0.8, Easing::Linear);
    emo.motion_reverse("base");
    emo.on_end(id, |emo| {
        emo.on_delay(1.5, |emo| {
            reset_layers_and_start_anim(emo);
        });
    });
}

/// Resets layers and begins idle loop tweens and face movements.
fn reset_layers_and_start_anim(emo: &mut EmotivaHeart) {
    emo.reset_layer("eyes_open");
    emo.reset_layer("mouth_closed");
    emo.on_delay(0.5, |emo| {
        emo.eyes_start();
        emo.mouth_start();
        emo.tween_start("mouth_open");
        emo.tween_start("hair_front");
        emo.tween_start("hair_behind");
        emo.tween_start("base");
    });
}

/// Stops all idle animation and face motion.
/// Called after ~20s of the cycle.
fn stop_idle_cycle(emotiva: &mut Emotiva) {
    emotiva.tween_stop("base");
    emotiva.tween_stop("hair_behind");
    emotiva.tween_stop("hair_front");
    emotiva.tween_stop("mouth_open");
    emotiva.mouth_stop();
    emotiva.eyes_stop();
    emotiva.eyes_set_blink_duration(1.0);
    emotiva.eyes_set_blink_interval_range((2.0, 4.0));
    emotiva.mouth_set_talk_interval(1.0);
    emotiva.mouth_set_talk_duration(1.2);
    emotiva.mouth_set_flap_open_time(0.2);
}

/// Draws the background dojo image.
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
