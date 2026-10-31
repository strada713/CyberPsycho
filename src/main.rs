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
    rand::srand(macroquad::miniquad::date::now() as u64);
    let mut blockers: i32 = 0;
    let mut blockers_pos = vec2(0.0, 0.0);
    let mut blocker_on_map = false;
    let mut blocker_timer: f32 = 5.0;
    let mut kills = 0;
    let mut respawn_timer = 0.0;
    let mut floor = 1;
    let mut floor_kills = 0;

    loop {
        let mut dir = vec2(0.0, 0.0);
        let enemy_time = if sandy_left > 0.0 { 0.2 } else { 1.0 };
        let dt = get_frame_time();
        if !blocker_on_map {
            blocker_timer -= dt;
            if blocker_timer <= 0.0 {
                blockers_pos = vec2(rand::gen_range(50.0, 1230.0), rand::gen_range(50.0, 670.0));
                blocker_on_map = true;
            }
        }
        if blocker_on_map && pos.distance(blockers_pos) < 25.0 {
            blockers += 1;
            blocker_on_map = false;
            blocker_timer = 8.0;
        }
        if sandy_left > 0.0 {
            sandy_left -= dt;
        }
        if cooldown > 0.0 {
            cooldown -= dt;
        }
        if is_key_pressed(KeyCode::LeftShift) && sandy_left <= 0.0 && cooldown <= 0.0 {
            if blockers > 0 {
                blockers -= 1;
                sandy_left = 5.0;
                cooldown = 15.0;
            } else {
                game_over = true;
            }
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
        if is_mouse_button_pressed(MouseButton::Left) && enemy_alive && pos.distance(enemy) < 60.0 {
            enemy_alive = false;
            kills += 1;
            respawn_timer = 2.0;
            floor_kills += 1;
            if floor_kills >= 10 {
                floor += 1;
                floor_kills = 0;
            }
            if floor == 10 {
                let boss = floor >= 10;
            }
        }
        if !enemy_alive && !game_over {
            respawn_timer -= dt;
            if respawn_timer <= 0.0 {
                let x = if rand::gen_range(0, 2) == 0 {
                    -20.0
                } else {
                    1300.0
                };
                enemy = vec2(x, rand::gen_range(0.0, 720.0));
                enemy_alive = true;
            }
        }
        if !game_over {
            pos += dir * speed * get_frame_time();
            pos.x = pos.x.clamp(15.0, screen_width() - 15.0);
            pos.y = pos.y.clamp(15.0, screen_height() - 15.0);
            if enemy_alive {
                let current_speed = enemy_speed + floor as f32 * 12.0;
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
            kills = 0;
            blockers = 0;
            floor = 1;
            floor_kills = 0;
            respawn_timer = 0.0;
        }
        clear_background(BLACK);
        draw_circle(pos.x, pos.y, 15.0, SKYBLUE);
        draw_circle(mouse_position().0, mouse_position().1, 8.0, RED);
        let ready = 1.0 - cooldown / 15.0;
        draw_rectangle(15.0, 690.0, 200.0 * ready, 12.0, SKYBLUE);
        draw_text("Sandevistan", 20.0, 670.0, 35.0, GREEN);
        if enemy_alive {
            draw_circle(enemy.x, enemy.y, 15.0, YELLOW);
        }
        if game_over {
            draw_text("Cyberpsychosis", 400.0, 310.0, 60.0, RED);
            draw_text("Press R to restart", 400.0, 340.0, 40.0, WHITE);
        }
        if blocker_on_map {
            draw_circle(blockers_pos.x, blockers_pos.y, 10.0, GREEN);
        }
        draw_text(&format!("Blockers: {}", blockers), 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Kills {}", kills), 20.0, 60.0, 30.0, WHITE);
        draw_text(&format!("Floor: {}/10", floor), 1080.0, 30.0, 30.0, WHITE);
        draw_line(
            pos.x,
            pos.y,
            mouse_position().0,
            mouse_position().1,
            2.0,
            WHITE,
        );
        if sandy_left > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.6, 1.0, 0.15),
            );
        }
        next_frame().await;
    }
}
