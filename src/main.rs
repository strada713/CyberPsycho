use std::os::unix::process;

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

#[derive(PartialEq)]
enum State {
    Menu,
    Controls,
    Playing,
    Paused,
}
enum DeathCause {
    Enemy,
    Cyberpsychosis,
}

#[macroquad::main(window_conf)]
async fn main() {
    let speed: f32 = 250.0;
    let mut game_over = false;
    let start = vec2(640.0, 360.0);
    let mut pos = start;
    let enemy_speed = 120.0;
    let enemy_start = vec2(100.0, 100.0);
    let mut enemy = enemy_start;
    let mut enemy_alive = true;
    let mut sandy_left: f32 = 0.0;
    let mut cooldown: f32 = 0.0;
    let mut state = State::Menu;
    rand::srand(macroquad::miniquad::date::now() as u64);
    let mut blockers: i32 = 0;
    let mut blockers_pos = vec2(0.0, 0.0);
    let mut blocker_on_map = false;
    let mut blocker_timer: f32 = 5.0;
    let mut kills = 0;
    let mut respawn_timer = 0.0;
    let mut floor = 1;
    let mut floor_kills = 0;
    let hit_sound = load_sound("assets/hit.wav").await.unwrap();
    let sandy_sound = load_sound("assets/sandy.wav").await.unwrap();
    let music = load_sound("assets/music.wav").await.unwrap();
    let death_sound = load_sound("assets/Death.wav").await.unwrap();
    let mut banner = 0.0;
    let boss_music = load_sound("assets/boss.wav").await.unwrap();
    let mut flash = 0.0;
    let mut dash_left = 0.0;
    let mut dash_cd = 0.0;
    let mut dash_dir = vec2(0.0, 0.0);
    let pickup_sound = load_sound("assets/pick.wav").await.unwrap();
    let whosh_sound = load_sound("assets/whosh.wav").await.unwrap();
    let mut selected: usize = 0;
    let player_tex = load_texture("assets/player.png").await.unwrap();
    let player_sheet = load_texture("assets/sheet.png").await.unwrap();
    player_sheet.set_filter(FilterMode::Nearest);
    let frames = 4.0;
    let mut attack_t = 0.0;
    let enemy_tex = load_texture("assets/enemy.png").await.unwrap();
    enemy_tex.set_filter(FilterMode::Nearest);
    player_tex.set_filter(FilterMode::Nearest);
    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 0.5,
        },
    );

    loop {
        if state == State::Menu {
            clear_background(BLACK);

            let title = "CYBERPSYCHO";
            let tw = measure_text(title, None, 90, 1.0).width;
            draw_text(
                title,
                (screen_width() - tw) / 2.0 + 4.0,
                224.0,
                90.0,
                MAGENTA,
            );
            draw_text(title, (screen_width() - tw) / 2.0, 220.0, 90.0, SKYBLUE);

            let items = ["Start", "Controls", "Quit"];

            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                selected = (selected + 1) % items.len();
            }
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                selected = (selected + items.len() - 1) % items.len();
            }

            let mut y = 360.0;
            for (i, item) in items.iter().enumerate() {
                let label = if i == selected {
                    format!("> {} <", item)
                } else {
                    item.to_string()
                };
                let color = if i == selected { WHITE } else { GRAY };
                let w = measure_text(&label, None, 50, 1.0).width;
                draw_text(&label, (screen_width() - w) / 2.0, y, 50.0, color);
                y += 70.0;
            }

            if is_key_pressed(KeyCode::Enter) {
                if selected == 0 {
                    state = State::Playing;
                } else if selected == 1 {
                    state = State::Controls;
                } else {
                    break;
                }
            }

            next_frame().await;
            continue;
        }

        if state == State::Controls {
            clear_background(BLACK);

            let header = "CONTROLS";
            let hw = measure_text(header, None, 70, 1.0).width;
            draw_text(header, (screen_width() - hw) / 2.0, 150.0, 70.0, SKYBLUE);

            let lines = [
                "WASD - move",
                "Mouse aim + LMB - strike",
                "SPACE - dash",
                "SHIFT - Sandevistan (uses a blocker)",
                "P - pause",
            ];
            let mut y = 260.0;
            for line in lines {
                let w = measure_text(line, None, 32, 1.0).width;
                draw_text(line, (screen_width() - w) / 2.0, y, 32.0, LIGHTGRAY);
                y += 45.0;
            }

            let warn = "No blocker = Cyberpsychosis!";
            let ww = measure_text(warn, None, 32, 1.0).width;
            draw_text(warn, (screen_width() - ww) / 2.0, y + 20.0, 32.0, RED);

            let back = "ENTER - back";
            let bw = measure_text(back, None, 28, 1.0).width;
            draw_text(back, (screen_width() - bw) / 2.0, 640.0, 28.0, GRAY);

            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Escape) {
                state = State::Menu;
            }

            next_frame().await;
            continue;
        }

        if is_key_pressed(KeyCode::P) && !game_over {
            if state == State::Playing {
                state = State::Paused;
            } else if state == State::Paused {
                state = State::Playing;
            }
        }
        if state == State::Paused {
            let text = "PAUSED";
            let w = measure_text(text, None, 80, 1.0).width;
            draw_text(text, (screen_width() - w) / 2.0, 360.0, 80.0, WHITE);
            next_frame().await;
            continue;
        }

        let mut dir = vec2(0.0, 0.0);
        let enemy_time = if sandy_left > 0.0 { 0.2 } else { 1.0 };
        let dt = get_frame_time();
        let was_over = game_over;
        let boss = floor >= 10;
        let enemy_radius = if boss { 60.0 } else { 22.0 };
        if banner > 0.0 {
            banner -= dt;
        }
        if flash > 0.0 {
            flash -= dt;
        }
        if dash_left > 0.0 {
            dash_left -= dt;
        }
        if dash_cd > 0.0 {
            dash_cd -= dt;
        }
        if attack_t > 0.0 {
            attack_t -= dt;
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
            play_sound_once(&pickup_sound);
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
        if is_key_pressed(KeyCode::Space) && dash_cd <= 0.0 && !game_over {
            dash_left = 0.15;
            dash_cd = 1.5;
            dash_dir = aim;
            play_sound_once(&whosh_sound);
        }
        let clicked = is_mouse_button_pressed(MouseButton::Left) && attack_t <= 0.0 && !game_over;
        if clicked {
            attack_t = 0.3;
            if enemy_alive && !boss && pos.distance(enemy) < 80.0 && aim.dot(to_enemy) > 0.5 {
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
            if dash_left > 0.0 {
                pos += dash_dir * speed * 4.0 * get_frame_time();
            } else {
                pos += dir * speed * get_frame_time();
            }
            pos.x = pos.x.clamp(22.0, screen_width() - 22.0);
            pos.y = pos.y.clamp(22.0, screen_height() - 22.0);
            if enemy_alive {
                let current_speed =
                    enemy_speed + floor as f32 * 12.0 + if boss { 80.0 } else { 0.0 };
                let to_player = (pos - enemy).normalize();
                enemy += to_player * current_speed * get_frame_time() * enemy_time;
            }
        }
        if enemy_alive && pos.distance(enemy) < 22.0 + enemy_radius {
            game_over = true;
        }
        if game_over && !was_over {
            play_sound_once(&death_sound);
            flash = 0.6;
        }
        if game_over && (is_key_pressed(KeyCode::R) || is_key_pressed(KeyCode::Escape)) {
            if is_key_pressed(KeyCode::Escape) {
                state = State::Menu;
            }
            game_over = false;
            pos = start;
            enemy = enemy_start;
            enemy_alive = true;
            sandy_left = 0.0;
            cooldown = 0.0;
            kills = 0;
            blockers = 0;
            blocker_on_map = false;
            blocker_timer = 5.0;
            floor = 1;
            floor_kills = 0;
            respawn_timer = 0.0;
            banner = 0.0;
            flash = 0.0;
            dash_left = 0.0;
            dash_cd = 0.0;
            stop_sound(&death_sound);
            stop_sound(&boss_music);
            stop_sound(&music);
            play_sound(
                &music,
                PlaySoundParams {
                    looped: true,
                    volume: 0.5,
                },
            );
        }

        clear_background(BLACK);
        let angle = aim.y.atan2(aim.x);
        let frame = if attack_t > 0.0 {
            let progress = 1.0 - attack_t / 0.3;
            (1.0 + (progress * (frames - 1.0)).floor()).min(frames - 1.0)
        } else {
            0.0
        };
        draw_texture_ex(
            &player_sheet,
            pos.x - 48.0,
            pos.y - 48.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(96.0, 96.0)),
                source: Some(Rect::new(frame * 48.0, 0.0, 48.0, 48.0)),
                rotation: angle,
                ..Default::default()
            },
        );
        draw_circle(mouse_position().0, mouse_position().1, 8.0, RED);
        let ready = 1.0 - cooldown / 15.0;
        draw_rectangle(15.0, 690.0, 200.0 * ready, 12.0, SKYBLUE);
        draw_text("Sandevistan", 20.0, 670.0, 35.0, GREEN);
        if enemy_alive {
            let look = (pos - enemy).normalize_or_zero();
            let enemy_angle = look.y.atan2(look.x);
            let size = if boss { 192.0 } else { 96.0 };
            let tint = if boss { PURPLE } else { WHITE };
            draw_texture_ex(
                &enemy_tex,
                enemy.x - size / 2.0,
                enemy.y - size / 2.0,
                tint,
                DrawTextureParams {
                    dest_size: Some(vec2(size, size)),
                    rotation: enemy_angle,
                    ..Default::default()
                },
            );
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
