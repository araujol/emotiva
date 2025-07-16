use ::rand::rng;
use emotiva::CharAnimator;
use emotiva::format::load_rig_from_file;

use macroquad::prelude::*;

use std::collections::HashMap;

#[macroquad::main("Emotiva Viewer")]
async fn main() {
    // Load rig
    let rig =
        load_rig_from_file("test_data/example.emotiva.ron").expect("Failed to load .ron rig file");

    let mut rng = rng();
    let mut animator = CharAnimator::new(rig, &mut rng);

    // Load all textures
    let mut textures: HashMap<String, Texture2D> = HashMap::new();
    for layer in &animator.rig.layers {
        let tex = load_texture(&format!("test_data/{}", layer.image))
            .await
            .unwrap();
        textures.insert(layer.image.clone(), tex);
    }

    loop {
        clear_background(BLUE);

        let delta = get_frame_time();
        animator.update(delta, &mut rng);

        for sprite in animator.get_drawables() {
            if let Some(tex) = textures.get(&sprite.image) {
                let params = DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        tex.width() * sprite.scale,
                        tex.height() * sprite.scale,
                    )),
                    ..Default::default()
                };
                draw_texture_ex(tex, sprite.position.0, sprite.position.1, WHITE, params);
            }
        }

        next_frame().await;
    }
}
