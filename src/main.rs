use macroquad::prelude::*;

#[macroquad::main("CyberPsycho")]
async fn main() {
    let mut x: f32 = 400.0;
    let mut y: f32 = 300.0;
    let mut pos = vec2(400.0, 300.0);
    let mut speed: f32 = 250.0;
    let mouse: Vec2 = mouse_position().into();

    loop {
        let mut dir = vec2(0.0, 0.0);
        if is_key_down(KeyCode::D) {
            dir.x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            dir.x -= 1.0;
        }
        if is_key_down(KeyCode::W) {
            dir.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            dir.y += 1.0;
            if dir != Vec2::ZERO {
                dir = dir.normalize();
            }
        }
        pos += dir * speed * get_frame_time();
        clear_background(BLACK);
        draw_rectangle(pos.x, pos.y, 30.0, 30.0, SKYBLUE);
        draw_circle(pos.x, pos.y, 15.0, SKYBLUE);
        draw_circle(mouse_position().0, mouse_position().1, 8.0, RED);
        draw_line(
            pos.x,
            pos.y,
            mouse_position().0,
            mouse_position().1,
            2.0,
            WHITE,
        );
        next_frame().await;
    }
}
