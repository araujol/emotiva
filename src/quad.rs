use crate::CharAnimator;
use crate::format::load_rig_from_file;

// Use crate rand from root to avoid Macroquad's re-export conflict
use ::rand::rng;
use ::rand::rngs::ThreadRng;
use std::collections::HashMap;

use macroquad::prelude::*;

pub struct EmotivaQuad {
    animator: CharAnimator,
    textures: HashMap<String, Texture2D>,
    rng: ThreadRng,
    base_position: Vec2,
}

impl EmotivaQuad {
    pub async fn load(path: &str, texture_base_path: &str) -> Self {
        let rig = load_rig_from_file(path).expect("Failed to load .ron rig file");
        let mut rng = rng();
        let animator = CharAnimator::new(rig, &mut rng);

        let mut textures: HashMap<String, Texture2D> = HashMap::new();
        for layer in &animator.rig.layers {
            let tex = load_texture(&format!("{}/{}", texture_base_path, layer.image))
                .await
                .unwrap();
            textures.insert(layer.image.clone(), tex);
        }

        EmotivaQuad {
            animator,
            textures,
            rng,
            base_position: Vec2::ZERO,
        }
    }

    pub fn set_base_position(&mut self, pos: Vec2) {
        self.base_position = pos;
    }

    pub fn update(&mut self, dt: f32) {
        self.animator.update(dt, &mut self.rng);
    }

    pub fn draw(&self) {
        for sprite in self.animator.get_drawables() {
            if let Some(tex) = self.textures.get(&sprite.image) {
                let pos_x = sprite.position.0 + self.base_position.x;
                let pos_y = sprite.position.1 + self.base_position.y;

                let params = DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        tex.width() * sprite.scale,
                        tex.height() * sprite.scale,
                    )),
                    ..Default::default()
                };
                draw_texture_ex(tex, pos_x, pos_y, WHITE, params);
            }
        }
    }
}
