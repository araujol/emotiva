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
                        emo.set_layer("eyes", "delighted");
                        let id = emo.set_alpha("base", 0.0, 1.0, 0.5, Easing::SineInOut);
                        emo.on_end(id, |emo| {
                            let id = emo.set_scale("base", 1.0, 1.2, 0.5, Easing::Linear);
                            emo.on_end(id, |emo| {
                                let id = emo.set_scale("base", 1.2, 1.0, 0.8, Easing::Linear);
                                emo.on_end(id, |emo| {
                                    emo.on_delay(1.5, |emo| {
                                        emo.reset_layer("eyes");
                                        emo.trigger("eyes", "start_blinking");
                                        emo.trigger("mouth", "idle_chat");
                                        emo.tween_start("mouth");
                                        emo.tween_start("hair_front");
                                        emo.tween_start("hair_behind");
                                        emo.tween_start("base");
                                    });
                                });
                            });
                        });
                    });
                }
            } else {
                if i >= 10 {
                    emotiva.tween_stop_easing("base");
                    emotiva.tween_stop_easing("hair_behind");
                    emotiva.tween_stop_easing("hair_front");
                    emotiva.tween_stop("mouth");
                    emotiva.trigger("mouth", "stop_talking");
                    emotiva.trigger("eyes", "stop_blinking");
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
