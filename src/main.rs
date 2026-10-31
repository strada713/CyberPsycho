use macroquad::audio::{PlaySoundParams, load_sound, play_sound, play_sound_once, stop_sound};
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
    let mut floor = 9;
    let mut floor_kills = 0;
    let hit_sound = load_sound("assets/hit.wav").await.unwrap();
    let sandy_sound = load_sound("assets/sandy.wav").await.unwrap();
    let music = load_sound("assets/music.wav").await.unwrap();
    let death_sound = load_sound("assets/Death.wav").await.unwrap();
    let mut banner = 0.0;
    let boss_music = load_sound("assets/boss.wav").await.unwrap();
    let mut flash = 0.0;
    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 0.5,
        },
    );

    loop {
        let mut dir = vec2(0.0, 0.0);
        let enemy_time = if sandy_left > 0.0 { 0.2 } else { 1.0 };
        let dt = get_frame_time();
        let was_over = game_over;
        let boss = floor >= 10;
        let enemy_radius = if boss { 45.0 } else { 15.0 };
        if banner > 0.0 {
            banner -= dt;
        }
        if flash > 0.0 {
            flash -= dt;
        }

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
                play_sound_once(&sandy_sound);
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
        let aim = (Vec2::from(mouse_position()) - pos).normalize_or_zero();
        let to_enemy = (enemy - pos).normalize_or_zero();
        if is_mouse_button_pressed(MouseButton::Left)
            && enemy_alive
            && !boss
            && pos.distance(enemy) < 60.0
            && aim.dot(to_enemy) > 0.5
        {
            enemy_alive = false;
            kills += 1;
            play_sound_once(&hit_sound);
            respawn_timer = 2.0;
            floor_kills += 1;
            if floor_kills >= 10 {
                floor += 1;
                floor_kills = 0;
                banner = 2.0;
                if floor == 10 {
                    stop_sound(&music);
                    play_sound(
                        &boss_music,
                        PlaySoundParams {
                            looped: true,
                            volume: 0.5,
                        },
                    );
                }
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
                let current_speed =
                    enemy_speed + floor as f32 * 12.0 + if boss { 80.0 } else { 0.0 };
                let to_player = (pos - enemy).normalize();
                enemy += to_player * current_speed * get_frame_time() * enemy_time;
            }
        }
        if enemy_alive && pos.distance(enemy) < 15.0 + enemy_radius {
            game_over = true;
        }
        if game_over && !was_over {
            play_sound_once(&death_sound);
            flash = 0.6;
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
            stop_sound(&death_sound);
            banner = 0.0;
            stop_sound(&boss_music);
        }
        clear_background(BLACK);
        draw_circle(pos.x, pos.y, 15.0, SKYBLUE);
        draw_circle(mouse_position().0, mouse_position().1, 8.0, RED);
        let ready = 1.0 - cooldown / 15.0;
        draw_rectangle(15.0, 690.0, 200.0 * ready, 12.0, SKYBLUE);
        draw_text("Sandevistan", 20.0, 670.0, 35.0, GREEN);
        if enemy_alive {
            let color = if boss { PURPLE } else { YELLOW };
            draw_circle(enemy.x, enemy.y, enemy_radius, color);
        }
        if game_over {
            let title = "Cyberpsychosis";
            let sub = "Press R to restart";

            let w1 = measure_text(title, None, 60, 1.0).width;
            let w2 = measure_text(sub, None, 40, 1.0).width;
            draw_text(title, (screen_width() - w1) / 2.0, 340.0, 60.0, RED);
            draw_text(sub, (screen_width() - w2) / 2.0, 390.0, 40.0, WHITE);
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
        if banner > 0.0 {
            let text = format!("FLOOR {}", floor);
            let w = measure_text(&text, None, 90, 1.0).width;
            draw_text(&text, (screen_width() - w) / 2.0, 360.0, 90.0, WHITE);
        }
        if flash > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(1.0, 0.0, 0.0, flash / 0.6 * 0.5),
            );
        }
        next_frame().await;
    }
}
