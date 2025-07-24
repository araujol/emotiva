use emotiva::quad::EmotivaQuad;

use macroquad::prelude::*;

#[macroquad::main("Emotiva Viewer")]
async fn main() {
    let mut emotiva = EmotivaQuad::load("test_data/example.emotiva.ron", "test_data").await;

    let mut elapsed = 0.0;
    let mut toggled = true;

    loop {
        clear_background(GRAY);

        let delta = get_frame_time();

        elapsed += delta;
        if elapsed >= 10.0 {
            if toggled {
                println!("start!");
                emotiva.trigger("eyes", "start_blinking");
                emotiva.trigger("mouth", "start_talking");
                emotiva.tween_start("eyes");
                emotiva.tween_start("mouth");
                emotiva.tween_start("base");
            } else {
                println!("stop!");
                emotiva.trigger("eyes", "stop_blinking");
                emotiva.trigger("mouth", "stop_talking");
                emotiva.tween_stop("eyes");
                emotiva.tween_stop("base");
            }

            toggled = !toggled;
            elapsed = 0.0;
        }

        emotiva.update(delta);
        emotiva.draw();

        next_frame().await;
    }
}
