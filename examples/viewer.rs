use emotiva::quad::EmotivaQuad;

use macroquad::prelude::*;

#[macroquad::main("Emotiva Viewer")]
async fn main() {
    let mut emotiva = EmotivaQuad::load("test_data/example.emotiva.ron", "test_data").await;
    emotiva.set_base_position(Vec2::new(0.0, 0.0));

    //let mut elapsed = 0.0;
    //let mut toggled = false;

    loop {
        clear_background(GRAY);

        let delta = get_frame_time();

        /*
        elapsed += delta;
        if elapsed >= 10.0 {
            if toggled {
                emotiva.reset_image("base");
            } else {
                emotiva.set_image("base", "hello");
            }
            toggled = !toggled;
            elapsed = 0.0;
        } */

        emotiva.update(delta);
        emotiva.draw();

        next_frame().await;
    }
}
