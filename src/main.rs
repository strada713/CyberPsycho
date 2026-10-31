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
    let speed: f32 = 250.0;
    let mut game_over = false;
    let start = vec2(640.0, 360.0);
    let mut pos = start;
    let enemy_speed = 120.0;
    let ememy_start = vec2(100.0, 100.0);
    let mut enemy = ememy_start;
    let mut enemy_alive = true;
    let mut sandy_left: f32 = 0.0;
    let mut cooldown: f32 = 0.0;

    loop {
        let mut dir = vec2(0.0, 0.0);
        let enemy_time = if sandy_left > 0.0 { 0.2 } else { 1.0 };
        let dt = get_frame_time();
        if sandy_left > 0.0 {
            sandy_left -= dt;
        }
        if cooldown > 0.0 {
            cooldown -= dt;
        }
        if is_key_pressed(KeyCode::LeftShift) && sandy_left <= 0.0 && cooldown <= 0.0 {
            sandy_left = 5.0;
            cooldown = 15.0;
        }
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
        if is_mouse_button_pressed(MouseButton::Left) && pos.distance(enemy) < 60.0 {
            enemy_alive = false;
        }
        if !game_over {
            pos += dir * speed * get_frame_time();
            if enemy_alive {
                let to_player = (pos - enemy).normalize();
                enemy += to_player * enemy_speed * get_frame_time() * enemy_time;
            }
        }
        if enemy_alive && pos.distance(enemy) < 30.0 {
            game_over = true;
        }
        if game_over && is_key_pressed(KeyCode::R) {
            game_over = false;
            pos = start;
            enemy = ememy_start;
            enemy_alive = true;
            sandy_left = 0.0;
            cooldown = 0.0;
        }
        clear_background(BLACK);
        draw_circle(pos.x, pos.y, 15.0, SKYBLUE);
        draw_circle(mouse_position().0, mouse_position().1, 8.0, RED);
        if enemy_alive {
            draw_circle(enemy.x, enemy.y, 15.0, YELLOW);
        }
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
