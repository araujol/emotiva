use emotiva::quad::EmotivaQuad;

use macroquad::prelude::*;

fn move_char(emotiva: &mut EmotivaQuad) {
    emotiva.trigger("base", "motion_play");
    emotiva.trigger("eyes", "motion_play");
    emotiva.trigger("mouth", "motion_play");
}

fn reverse_char(emotiva: &mut EmotivaQuad) {
    emotiva.trigger("base", "motion_reverse");
    emotiva.trigger("eyes", "motion_reverse");
    emotiva.trigger("mouth", "motion_reverse");
}

#[macroquad::main("Emotiva Viewer")]
async fn main() {
    let mut emotiva = EmotivaQuad::load("test_data/example.emotiva.ron", "test_data").await;
    emotiva.set_base_position(Vec2::new(0.0, 0.0));

    let mut elapsed = 0.0;
    let mut toggled = true;

    loop {
        clear_background(GRAY);

        let delta = get_frame_time();

        elapsed += delta;
        if elapsed >= 5.0 {
            if toggled {
                println!("start!");
                emotiva.trigger("eyes", "start_blinking");
                emotiva.trigger("eyes", "tween_start");
                emotiva.trigger("mouth", "start_talking");
                emotiva.trigger("mouth", "tween_start");
                emotiva.trigger("base", "tween_start");
                move_char(&mut emotiva);
                //emotiva.trigger("mouth", "idle_chat");
                //emotiva.set_image("base", "hello");
            } else {
                println!("stop!");
                emotiva.trigger("eyes", "stop_blinking");
                emotiva.trigger("eyes", "tween_stop");
                emotiva.trigger("mouth", "stop_talking");
                emotiva.trigger("base", "tween_stop");
                reverse_char(&mut emotiva);
                //emotiva.reset_image("base");
            }

            toggled = !toggled;
            elapsed = 0.0;
        }

        emotiva.update(delta);
        emotiva.draw();

        next_frame().await;
    }
}
