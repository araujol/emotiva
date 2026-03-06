//! # Emotiva Nested Callback Example
//!
//! Demonstrates how to build a **multi-stage animation sequence**
//! using Emotiva's callback system (`on_end`, `on_delay`) together
//! with a simple external state loop.
//!
//! The example loads Rina’s rig and chains several animations to form
//! a small character performance, combining visual effects, motion,
//! facial changes, and idle tweens.
//!
//! ## What This Example Shows
//!
//! - Chaining animations using `on_end`
//! - Scheduling delayed actions using `on_delay`
//! - Mixing runtime callbacks with external state logic
//! - Structuring animation stages into helper functions
//! - Dynamically starting and stopping idle animations
//!
//! ## Animation Sequence
//!
//! The demo performs the following cycle:
//!
//! 1. Wait ~3 seconds before starting
//! 2. Fade out the body layer
//! 3. Change facial layers
//! 4. Fade back in, apply scale pulse, and play motion
//! 5. Reset layers and begin idle animations (eyes, mouth, hair, body)
//! 6. Run idle loop for ~60 seconds
//! 7. Stop all animation
//! 8. Return to the idle delay and repeat
//!
//! This example highlights how Emotiva supports combining
//! **declarative rig animations** (tweens and motions) with
//! **imperative runtime sequencing** through callbacks.
//!
//! ## Run
//!
//! ```bash
//! cargo run --example nested_callbacks
//! ```
use emotiva::Emotiva;
use emotiva::EmotivaHeart;
use emotiva::api::Easing;
use emotiva::api::EmotivaAPI;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Emotiva - Nested Animation Callbacks".to_string(),
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
        "examples/nested_callbacks/rina.emotiva.ron",
        "examples/assets/rina",
    )
    .await;

    let mut elapsed = 0.0;
    let mut state = 0; // 0: Idle delay, 1: Running, 2: Cooldown

    emotiva.set_screen_position((768.0, 480.0));

    loop {
        clear_background(WHITE);
        let delta = get_frame_time().min(0.1);
        elapsed += delta;

        match state {
            0 => {
                // Wait 3 seconds before first start
                if elapsed >= 3.0 && !emotiva.is_tween_enabled("body") {
                    start_intro_sequence(&mut emotiva);
                    state = 1; // switch to running
                    elapsed = 0.0;
                }
            }
            1 => {
                // Running: stop after 20 seconds
                if elapsed >= 60.0 {
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

/// Starts a sequence of introductory animations for rina.
/// This fades her out, changes some facial layers, then fades her back in.
fn start_intro_sequence(emotiva: &mut Emotiva) {
    let id = emotiva.set_alpha("body", 1.0, 0.0, 0.5, Easing::SineIn);
    emotiva.on_end(id, |emo| {
        intro_stage_two(emo);
    });
}

/// Second part of the intro animation.
/// Sets eyes/mouth layers, fades back in, applies scale pulse with motion.
fn intro_stage_two(emo: &mut EmotivaHeart) {
    emo.set_layer("eyes_open", "delighted");
    emo.set_layer("mouth_closed", "nothing");

    let id = emo.set_alpha("body", 0.0, 1.0, 0.5, Easing::SineInOut);
    emo.on_end(id, |emo| {
        let id = emo.set_scale("body", 1.0, 1.2, 0.8, Easing::Linear);
        emo.motion_play("body");
        emo.on_end(id, |emo| {
            intro_stage_three(emo);
        });
    });
}

/// Final part of the intro animation.
/// Shrinks back the scale, reverses motion, delays and resets layers.
fn intro_stage_three(emo: &mut EmotivaHeart) {
    let id = emo.set_scale("body", 1.2, 1.0, 0.8, Easing::Linear);
    emo.motion_reverse("body");
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
        emo.tween_start("name_tag");
        emo.tween_start("glasses");
        emo.tween_start("mouth_open");
        emo.tween_start("hair_front");
        emo.tween_start("hair_behind");
        emo.tween_start("coat");
        emo.tween_start("body");
    });
}

/// Stops all idle animation and face motion.
/// Called after ~20s of the cycle.
fn stop_idle_cycle(emotiva: &mut Emotiva) {
    emotiva.tween_stop("body");
    emotiva.tween_stop("coat");
    emotiva.tween_stop("hair_behind");
    emotiva.tween_stop("hair_front");
    emotiva.tween_stop("mouth_open");
    emotiva.tween_stop("glasses");
    emotiva.tween_stop("name_tag");
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
