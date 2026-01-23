use emotiva::Emotiva;
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
        Emotiva::load_with_textures("test_data/example_fuurin.emotiva.ron", "test_data").await;

    let mut elapsed = 0.0;
    let mut toggled = true;

    loop {
        clear_background(GRAY);

        let delta = get_frame_time();

        elapsed += delta;
        if elapsed >= 10.0 {
            if toggled {
                emotiva.tween_start("base");
            } else {
                emotiva.tween_stop("base");
            }
            toggled = !toggled;
            elapsed = 0.0;
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
