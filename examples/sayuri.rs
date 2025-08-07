use emotiva::api::EmotivaAPI;
use emotiva::easing::Easing;
use emotiva::quad::EmotivaQuad;

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
        EmotivaQuad::load("test_data/sayuri.emotiva.ron", "test_sprites/sayuri").await;

    let mut elapsed = 0.0;
    let mut toggled = true;
    let mut i = 0;

    loop {
        clear_background(WHITE);
        let delta = get_frame_time();

        elapsed += delta;
        if elapsed >= 5.0 {
            if toggled {
                if !emotiva.is_tween_enabled("base") {
                    // Nested callbacks
                    let id = emotiva.set_alpha("base", 1.0, 0.0, 0.5, Easing::SineIn);
                    emotiva.on_end(id, |emo| {
                        emo.set_layer("eyes_open", "delighted");
                        emo.set_layer("mouth_closed", "nothing"); // temporarily hide closed mouth
                        let id = emo.set_alpha("base", 0.0, 1.0, 0.5, Easing::SineInOut);
                        emo.on_end(id, |emo| {
                            let id = emo.set_scale("base", 1.0, 1.2, 0.8, Easing::Linear);
                            emo.motion_play("base");
                            emo.on_end(id, |emo| {
                                let id = emo.set_scale("base", 1.2, 1.0, 0.8, Easing::Linear);
                                emo.motion_reverse("base");
                                emo.on_end(id, |emo| {
                                    emo.on_delay(1.5, |emo| {
                                        emo.reset_layer("eyes_open");
                                        emo.reset_layer("mouth_closed");
                                        // brief delay before start talking
                                        emo.on_delay(0.5, |emo| {
                                            emo.eyes_start();
                                            emo.mouth_start();
                                            emo.tween_start("mouth_open");
                                            emo.tween_start("hair_front");
                                            emo.tween_start("hair_behind");
                                            emo.tween_start("base");
                                        });
                                    });
                                });
                            });
                        });
                    });
                }
            } else {
                if i >= 5 {
                    emotiva.tween_stop_easing("base");
                    emotiva.tween_stop_easing("hair_behind");
                    emotiva.tween_stop_easing("hair_front");
                    emotiva.tween_stop("mouth_open");
                    emotiva.mouth_stop();
                    emotiva.eyes_stop();
                    // Change eyes settings.
                    emotiva.eyes_set_blink_duration(1.0);
                    emotiva.eyes_set_blink_interval_range((2.0, 4.0));
                    // Change talk settins.
                    emotiva.mouth_set_talk_interval(1.0);
                    emotiva.mouth_set_talk_duration(1.2);
                    emotiva.mouth_set_flap_open_time(0.2);

                    i = 0;
                }
            }

            toggled = !toggled;
            elapsed = 0.0;
            i += 1;
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
