use emotiva::quad::EmotivaQuad;

use macroquad::prelude::*;

#[macroquad::main("Emotiva Viewer")]
async fn main() {
    let mut emotiva = EmotivaQuad::load("test_data/example.emotiva.ron", "test_data").await;
    emotiva.set_base_position(Vec2::new(0.0, 0.0));

    loop {
        clear_background(WHITE);

        let delta = get_frame_time();
        emotiva.update(delta);
        emotiva.draw();

        next_frame().await;
    }
}
