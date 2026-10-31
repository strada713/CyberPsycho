use macroquad::prelude::*;
fn window_conf() -> Conf {
    Conf {
        window_title: "CyberPsycho".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut speed: f32 = 250.0;
    let mut enemy = vec2(100.0, 100.0);
    let mut game_over = false;
    let start = vec2(640.0, 360.0);
    let mut pos = start;
    let enemy_speed = 120.0;
    let ememy_start = vec2(100.0, 100.0);
    let mut enemy = ememy_start;

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
        }
        if dir != Vec2::ZERO {
            dir = dir.normalize();
        }
        if !game_over {
            pos += dir * speed * get_frame_time();
            let to_player = (pos - enemy).normalize();
            enemy += to_player * enemy_speed * get_frame_time();
        }
        if pos.distance(enemy) < 30.0 {
            game_over = true;
        }
        if game_over && is_key_pressed(KeyCode::R) {
            game_over = false;
            pos = start;
            enemy = ememy_start;
        }
        clear_background(BLACK);
        draw_circle(pos.x, pos.y, 15.0, SKYBLUE);
        draw_circle(mouse_position().0, mouse_position().1, 8.0, RED);
        draw_circle(enemy.x, enemy.y, 15.0, YELLOW);
        if game_over {
            draw_text("Cyberpsychosis", 400.0, 310.0, 60.0, RED);
            draw_text("Press R to restart", 400.0, 340.0, 40.0, WHITE);
        }
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
