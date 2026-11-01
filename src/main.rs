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

struct Enemy {
    pos: Vec2,
}

struct Ghost {
    pos: Vec2,
    angle: f32,
    life: f32,
}
struct Blood {
    pos: Vec2,
    size: f32,
}
struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
}
struct Popup {
    pos: Vec2,
    text: String,
    life: f32,
}
#[macroquad::main(window_conf)]
async fn main() {
    let speed: f32 = 250.0;
    let mut game_over = false;
    let start = vec2(640.0, 360.0);
    let mut pos = start;
    let enemy_speed: f32 = 120.0;
    let mut sandy_left: f32 = 0.0;
    let mut cooldown: f32 = 0.0;
    let mut state = State::Menu;
    rand::srand(macroquad::miniquad::date::now() as u64);
    let mut blockers: i32 = 0;
    let mut blockers_pos = vec2(0.0, 0.0);
    let mut blocker_on_map = false;
    let mut blocker_timer: f32 = 5.0;
    let mut kills: i32 = 0;
    let mut combo = 0;
    let mut combo_timer = 0.0;
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut head_exploded = false;
    let mut spawn_timer: f32 = 1.0;
    let mut floor: i32 = 1;
    let mut blood: Vec<Blood> = Vec::new();
    let mut popups: Vec<Popup> = Vec::new();
    let mut floor_kills: i32 = 0;
    let mut ghosts: Vec<Ghost> = Vec::new();
    let mut ghost_timer: f32 = 0.0;
    let mut banner: f32 = 0.0;
    let mut flash: f32 = 0.0;
    let mut dash_left: f32 = 0.0;
    let mut dash_cd: f32 = 0.0;
    let mut dash_dir = vec2(0.0, 0.0);
    let mut selected: usize = 0;
    let mut attack_t: f32 = 0.0;
    let mut walk_t: f32 = 0.0;
    let mut death_t: f32 = 0.0;
    let mut death_angle: f32 = 0.0;
    let mut hitstop: f32 = 0.0;
    let mut psycho = 0.0;
    let crosshair = load_texture("assets/crosshair.png").await.unwrap();
    crosshair.set_filter(FilterMode::Nearest);
    let crosshair_red = load_texture("assets/crosshair_red.png").await.unwrap();
    crosshair_red.set_filter(FilterMode::Nearest);

    // sounds
    let hit_sound = load_sound("assets/hit.wav").await.unwrap();
    let sandy_sound = load_sound("assets/sandy.wav").await.unwrap();
    let music = load_sound("assets/music.wav").await.unwrap();
    let boss_music = load_sound("assets/boss.wav").await.unwrap();
    let death_sound = load_sound("assets/Death.wav").await.unwrap();
    let pickup_sound = load_sound("assets/pick.wav").await.unwrap();
    let whosh_sound = load_sound("assets/whosh.wav").await.unwrap();
    let enemy_hit_sound = load_sound("assets/enemy_hit.wav").await.unwrap();

    // images
    let player_sheet = load_texture("assets/sheet.png").await.unwrap();
    player_sheet.set_filter(FilterMode::Nearest);
    let frames: f32 = 4.0;

    let walk_sheet = load_texture("assets/walk.png").await.unwrap();
    walk_sheet.set_filter(FilterMode::Nearest);
    let walk_frames: f32 = 4.0;

    let death_sheet = load_texture("assets/death.png").await.unwrap();
    death_sheet.set_filter(FilterMode::Nearest);
    let death_frames: f32 = (death_sheet.width() / 48.0).floor();

    let enemy_tex = load_texture("assets/enemy.png").await.unwrap();
    enemy_tex.set_filter(FilterMode::Nearest);

    let chip_tex = load_texture("assets/chip.png").await.unwrap();
    chip_tex.set_filter(FilterMode::Nearest);

    let bg = load_texture("assets/Wallpaper.png").await.unwrap();
    bg.set_filter(FilterMode::Nearest);

    let sandy_icon: Option<Texture2D> = load_texture("assets/sandy_icon.png").await.ok();
    if let Some(icon) = &sandy_icon {
        icon.set_filter(FilterMode::Nearest);
    }

    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 0.5,
        },
    );
    show_mouse(false);

    loop {
        // menu
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

        // controls
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

        // pause
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

        // update
        let mut dir = vec2(0.0, 0.0);
        let enemy_time = if sandy_left > 0.0 { 0.2 } else { 1.0 };
        let raw_dt = get_frame_time();
        let dt = if hitstop > 0.0 {
            hitstop -= raw_dt;
            0.0
        } else {
            raw_dt
        };
        let was_over = game_over;
        let boss = floor >= 10;
        let enemy_radius = if boss { 60.0 } else { 22.0 };

        // timers
        if banner > 0.0 {
            banner -= dt;
        }
        if flash > 0.0 {
            flash -= dt;
        }
        // particles
        for p in particles.iter_mut() {
            p.pos += p.vel * dt;
            p.vel *= 1.0 - 3.0 * dt;
            p.life -= dt;
        }
        particles.retain(|p| p.life > 0.0);
        // popups
        for p in popups.iter_mut() {
            p.pos.y -= 40.0 * dt;
            p.life -= dt;
        }
        popups.retain(|p| p.life > 0.0);
        if dash_left > 0.0 {
            dash_left -= dt;
        }
        if dash_cd > 0.0 {
            dash_cd -= dt;
        }
        if attack_t > 0.0 {
            attack_t -= dt;
        }
        if sandy_left > 0.0 {
            sandy_left -= dt;
        }
        if cooldown > 0.0 {
            cooldown -= dt;
        }
        if combo_timer > 0.0 {
            combo_timer -= dt;
            if combo_timer <= 0.0 {
                combo = 0;
            }
        }
        // psycho
        if sandy_left > 0.0 {
            psycho += 15.0 * dt;
        } else {
            psycho -= 2.0 * dt;
        }
        psycho = psycho.clamp(0.0, 100.0);
        if psycho >= 100.0 {
            game_over = true;
        }

        // blockers
        if !blocker_on_map {
            blocker_timer -= dt;
            if blocker_timer <= 0.0 {
                blockers_pos = vec2(rand::gen_range(50.0, 1230.0), rand::gen_range(50.0, 670.0));
                blocker_on_map = true;
            }
        }
        if blocker_on_map && pos.distance(blockers_pos) < 35.0 {
            blockers += 1;
            psycho = (psycho - 10.0).max(0.0);
            blocker_on_map = false;
            play_sound_once(&pickup_sound);
            blocker_timer = 8.0;
        }

        // sandevistan
        if is_key_pressed(KeyCode::LeftShift) && sandy_left <= 0.0 && cooldown <= 0.0 {
            if blockers > 0 {
                blockers -= 1;
                sandy_left = 5.0;
                play_sound_once(&sandy_sound);
                cooldown = 15.0;
            } else {
                game_over = true;
                head_exploded = true;
            }
        }

        // input
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
        if dir != Vec2::ZERO && !game_over {
            walk_t += dt;
        } else {
            walk_t = 0.0;
        }

        let aim = (Vec2::from(mouse_position()) - pos).normalize_or_zero();

        // ghosts
        if sandy_left > 0.0 && !game_over {
            ghost_timer -= dt;
            if ghost_timer <= 0.0 {
                ghosts.push(Ghost {
                    pos,
                    angle: aim.y.atan2(aim.x),
                    life: 0.4,
                });
                ghost_timer = 0.04;
            }
        }
        for g in ghosts.iter_mut() {
            g.life -= dt;
        }
        ghosts.retain(|g| g.life > 0.0);

        // dash
        if is_key_pressed(KeyCode::Space) && dash_cd <= 0.0 && !game_over {
            dash_left = 0.15;
            dash_cd = 1.5;
            dash_dir = aim;
            play_sound_once(&whosh_sound);
        }

        // attack
        let clicked = is_mouse_button_pressed(MouseButton::Left) && attack_t <= 0.0 && !game_over;
        if clicked {
            attack_t = 0.3;
            if !boss {
                let before = enemies.len();
                enemies.retain(|e| {
                    let to_e = (e.pos - pos).normalize_or_zero();
                    let hit = pos.distance(e.pos) < 80.0 && aim.dot(to_e) > 0.5;
                    if hit {
                        for _ in 0..6 {
                            blood.push(Blood {
                                pos: e.pos
                                    + vec2(
                                        rand::gen_range(-25.0, 25.0),
                                        rand::gen_range(-25.0, 25.0),
                                    ),
                                size: rand::gen_range(4.0, 14.0),
                            });
                        }
                    }
                    !hit
                });
                let killed = before - enemies.len();
                if killed > 0 {
                    kills += killed as i32;
                    hitstop = 0.06;
                    combo += 1;
                    combo_timer = 2.0;
                    floor_kills += killed as i32;
                    play_sound_once(&hit_sound);

                    if floor_kills >= 10 {
                        floor += 1;
                        floor_kills = 0;
                        banner = 2.0;
                        if floor == 10 {
                            enemies.clear();
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
        }
        // blood
        if blood.len() > 300 {
            blood.drain(0..50);
        }
        // spawn
        if !game_over {
            let max_enemies = if boss { 1 } else { 1 + floor as usize / 2 };
            if enemies.len() < max_enemies {
                spawn_timer -= dt;
                if spawn_timer <= 0.0 {
                    let x = if rand::gen_range(0, 2) == 0 {
                        -20.0
                    } else {
                        1300.0
                    };
                    enemies.push(Enemy {
                        pos: vec2(x, rand::gen_range(0.0, 720.0)),
                    });
                    spawn_timer = 2.0;
                }
            }
        }

        // motion
        if !game_over {
            if dash_left > 0.0 {
                pos += dash_dir * speed * 4.0 * dt;
            } else {
                pos += dir * speed * dt;
            }
            pos.x = pos.x.clamp(22.0, screen_width() - 22.0);
            pos.y = pos.y.clamp(22.0, screen_height() - 22.0);

            let current_speed = enemy_speed + floor as f32 * 12.0 + if boss { 80.0 } else { 0.0 };
            for e in enemies.iter_mut() {
                let to_player = (pos - e.pos).normalize_or_zero();
                e.pos += to_player * current_speed * dt * enemy_time;
            }
        }

        // collision
        if !game_over
            && enemies
                .iter()
                .any(|e| pos.distance(e.pos) < 22.0 + enemy_radius)
        {
            game_over = true;
        }
        if game_over && !was_over {
            play_sound_once(&death_sound);
            play_sound_once(&enemy_hit_sound);
            flash = 0.6;
            death_t = 0.0;
            death_angle = aim.y.atan2(aim.x);
            if head_exploded {
                for _ in 0..40 {
                    let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                    let speed = rand::gen_range(100.0, 400.0);
                    particles.push(Particle {
                        pos: pos,
                        vel: vec2(angle.cos(), angle.sin()) * speed,
                        life: rand::gen_range(0.5, 1.2),
                    });
                }
            }
        }
        if game_over {
            death_t += dt;
        }

        // restart
        if game_over && (is_key_pressed(KeyCode::R) || is_key_pressed(KeyCode::Escape)) {
            if is_key_pressed(KeyCode::Escape) {
                state = State::Menu;
            }
            game_over = false;
            pos = start;
            sandy_left = 0.0;
            cooldown = 0.0;
            kills = 0;
            blockers = 0;
            blocker_on_map = false;
            blocker_timer = 5.0;
            floor = 1;
            floor_kills = 0;
            enemies.clear();
            ghosts.clear();
            spawn_timer = 1.0;
            banner = 0.0;
            flash = 0.0;
            dash_left = 0.0;
            dash_cd = 0.0;
            attack_t = 0.0;
            walk_t = 0.0;
            death_t = 0.0;
            combo = 0;
            combo_timer = 0.0;
            psycho = 0.0;
            head_exploded = false;
            particles.clear();
            blood.clear();
            popups.clear();
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

        // draw
        clear_background(BLACK);
        draw_texture_ex(
            &bg,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        // background effects
        for b in &blood {
            draw_circle(b.pos.x, b.pos.y, b.size, Color::new(0.5, 0.0, 0.05, 0.7));
        }
        for p in &particles {
            let a = (p.life / 1.2).min(1.0);
            draw_circle(p.pos.x, p.pos.y, 5.0, Color::new(0.8, 0.0, 0.1, a));
        }

        // afterimages
        for g in &ghosts {
            let alpha = g.life / 0.4 * 0.5;
            draw_texture_ex(
                &player_sheet,
                g.pos.x - 48.0,
                g.pos.y - 48.0,
                Color::new(0.3, 0.8, 1.0, alpha),
                DrawTextureParams {
                    dest_size: Some(vec2(96.0, 96.0)),
                    source: Some(Rect::new(0.0, 0.0, 48.0, 48.0)),
                    rotation: g.angle,
                    ..Default::default()
                },
            );
        }

        // player
        let angle = aim.y.atan2(aim.x);
        let (tex, frame, rot) = if game_over {
            let f =
                ((death_t * 8.0) as usize).min((death_frames as usize).saturating_sub(1)) as f32;
            (&death_sheet, f, death_angle)
        } else if attack_t > 0.0 {
            let progress = 1.0 - attack_t / 0.3;
            (
                &player_sheet,
                (1.0 + (progress * (frames - 1.0)).floor()).min(frames - 1.0),
                angle,
            )
        } else if dir != Vec2::ZERO {
            (
                &walk_sheet,
                (((walk_t * 8.0) as usize) % (walk_frames as usize)) as f32,
                angle,
            )
        } else {
            (&player_sheet, 0.0, angle)
        };
        if !head_exploded {
            draw_texture_ex(
                tex,
                pos.x - 48.0,
                pos.y - 48.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(96.0, 96.0)),
                    source: Some(Rect::new(frame * 48.0, 0.0, 48.0, 48.0)),
                    rotation: rot,
                    ..Default::default()
                },
            );
        }
        // enemies
        for e in &enemies {
            let look = (pos - e.pos).normalize_or_zero();
            let enemy_angle = look.y.atan2(look.x);
            let size = if boss { 192.0 } else { 96.0 };
            let tint = if boss { PURPLE } else { WHITE };
            draw_texture_ex(
                &enemy_tex,
                e.pos.x - size / 2.0,
                e.pos.y - size / 2.0,
                tint,
                DrawTextureParams {
                    dest_size: Some(vec2(size, size)),
                    rotation: enemy_angle,
                    ..Default::default()
                },
            );
        }

        // chip
        if blocker_on_map {
            let chip_size = 48.0;
            draw_texture_ex(
                &chip_tex,
                blockers_pos.x - chip_size / 2.0,
                blockers_pos.y - chip_size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(chip_size, chip_size)),
                    ..Default::default()
                },
            );
        }
        for p in &popups {
            let a = (p.life / 0.8).min(1.0);
            draw_text(
                &p.text,
                p.pos.x,
                p.pos.y,
                32.0,
                Color::new(1.0, 0.9, 0.2, a),
            );
        }
        let m = Vec2::from(mouse_position());
        let on_enemy = enemies.iter().any(|e| e.pos.distance(m) < 40.0);
        let cross = if on_enemy { &crosshair_red } else { &crosshair };

        // crosshair
        let (mx, my) = mouse_position();
        let pulse = 32.0 + 4.0 * (get_time() as f32 * 6.0).sin();
        draw_texture_ex(
            cross,
            mx - pulse / 2.0,
            my - pulse / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(pulse, pulse)),
                ..Default::default()
            },
        );
        draw_line(
            pos.x,
            pos.y,
            mouse_position().0,
            mouse_position().1,
            2.0,
            WHITE,
        );

        // interface
        let ready = (1.0 - cooldown / 15.0).clamp(0.0, 1.0);
        draw_rectangle(15.0, 690.0, 200.0 * ready, 12.0, SKYBLUE);
        draw_rectangle(15.0, 710.0, 2.0 * psycho, 6.0, RED);
        if let Some(icon) = &sandy_icon {
            draw_texture_ex(
                icon,
                15.0,
                628.0,
                Color::new(1.0, 1.0, 1.0, 0.4 + 0.6 * ready),
                DrawTextureParams {
                    dest_size: Some(vec2(56.0, 56.0)),
                    ..Default::default()
                },
            );
        } else {
            draw_text("Sandevistan", 20.0, 670.0, 35.0, GREEN);
        }
        draw_text(&format!("Blockers: {}", blockers), 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Kills {}", kills), 20.0, 60.0, 30.0, WHITE);
        if combo >= 2 {
            draw_text(&format!("COMBO x{}", combo), 20.0, 90.0, 40.0, ORANGE);
        }
        draw_text(&format!("Floor: {}/10", floor), 1080.0, 30.0, 30.0, WHITE);

        // effects
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
        if game_over && death_t > 1.0 {
            let title = "Cyberpsychosis";
            let sub = "R - restart   ESC - menu";

            let w1 = measure_text(title, None, 60, 1.0).width;
            let w2 = measure_text(sub, None, 40, 1.0).width;
            draw_text(title, (screen_width() - w1) / 2.0, 340.0, 60.0, RED);
            draw_text(sub, (screen_width() - w2) / 2.0, 390.0, 40.0, WHITE);
        }
        // glitch
        let glitches = ((psycho - 50.0).max(0.0) / 5.0) as i32;
        for _ in 0..glitches {
            let y = rand::gen_range(0.0, screen_height());
            let h = rand::gen_range(2.0, 12.0);
            draw_rectangle(0.0, y, screen_width(), h, Color::new(1.0, 0.0, 0.3, 0.15));
        }

        next_frame().await;
    }
}
