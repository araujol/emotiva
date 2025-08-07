use emotiva::api::EmotivaAPI;
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
    let mut emotiva = EmotivaQuad::load("test_data/example.emotiva.ron", "test_sprites/yuki").await;
    //emotiva.set_base_position(Vec2::new(768.0, 512.0));

    let mut elapsed = 0.0;
    let mut toggled = true;
    let mut i = 0;

    loop {
        clear_background(GRAY);

        let delta = get_frame_time();

        elapsed += delta;
        if elapsed >= 5.0 {
            if toggled {
                if !emotiva.is_tween_enabled("base") {
                    emotiva.eyes_start();
                    emotiva.mouth_start();
                    emotiva.tween_start("eyes");
                    emotiva.tween_start("mouth");
                    emotiva.tween_start("base");
                }
            } else {
                if i >= 5 {
                    emotiva.tween_stop("base");
                    emotiva.tween_stop("mouth");
                    emotiva.tween_stop("eyes");
                    emotiva.mouth_stop();
                    emotiva.eyes_stop();
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
