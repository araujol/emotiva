use emotiva::quad::EmotivaQuad;

use macroquad::prelude::*;

#[macroquad::main("Emotiva Viewer")]
async fn main() {
    let mut emotiva = EmotivaQuad::load("test_data/example_fuurin.emotiva.ron", "test_data").await;

    let mut elapsed = 0.0;
    let mut toggled = true;

    loop {
        clear_background(GRAY);

        let delta = get_frame_time();

        elapsed += delta;
        if elapsed >= 10.0 {
            if toggled {
                emotiva.trigger("base", "tween_start_easing");
            } else {
                emotiva.trigger("base", "tween_stop_easing");
            }
            toggled = !toggled;
            elapsed = 0.0;
        }

        emotiva.update(delta);
        emotiva.draw();

        next_frame().await;
    }
}
