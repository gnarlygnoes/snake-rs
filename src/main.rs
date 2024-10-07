use macroquad::prelude::*;

#[macroquad::main("Snake")]
async fn main() {
    let tile_size: f32 = 30.0;
    let tiles_x: f32 = 30.0;
    let tiles_y: f32 = 20.0;
    request_new_screen_size(tile_size * tiles_x, tile_size * tiles_y);

    let mut player = Vec::new();

    player.push(Rect {
        x: 0.,
        y: 0.,
        w: tile_size,
        h: tile_size,
    });

    let mut direction = Vec2 { x: 1.0, y: 0.0 };

    let mut food_pos = tile_position(tile_size, tiles_x, tiles_y);

    let mut t: f32 = 0.0;
    let mut speed: f32 = 0.25;
    let mut player_score: i32 = 0;
    let mut level: i8 = 1;
    let mut game_active = true;
    let mut move_lock = false;

    loop {
        if game_active {
            clear_background(BLACK);

            // let t = 1.0 / get_fps() as f32;
            t += get_frame_time();
            if t > speed {
                t = 0.0;
                move_lock = false;

                // for i in 0..99.rev() {}

                // for i in 1..player.len() {
                let mut i = player.len() - 1;
                loop {
                    if i < 1 {
                        break;
                    }
                    // println!("{}", i);
                    player[i].x = player[i - 1].x;
                    player[i].y = player[i - 1].y;
                    i -= 1;
                }

                player[0].x += tile_size * direction.x;
                player[0].y += tile_size * direction.y;
            }

            // Collision with food.
            if player[0].x == food_pos.x && player[0].y == food_pos.y {
                player_score += 1;
                food_pos = tile_position(tile_size, tiles_x, tiles_y);
                level = set_level(player_score);
                speed = set_speed(level);

                player.push(Rect {
                    x: player[player.len() - 1].x,
                    y: player[player.len() - 1].y,
                    w: tile_size,
                    h: tile_size,
                })
            }

            for i in 4..player.len() - 1 {
                if player[0].x == player[i].x && player[0].y == player[i].y {
                    game_active = false;
                }
            }
            if player[0].x >= tile_size * tiles_x || player[0].x < 0. {
                game_active = false;
            }
            if player[0].y >= tile_size * tiles_y || player[0].y < 0. {
                game_active = false;
            }

            direction = player_movement(direction, move_lock);
            if is_key_pressed(KeyCode::Left)
                || is_key_pressed(KeyCode::Right)
                || is_key_pressed(KeyCode::Up)
                || is_key_pressed(KeyCode::Down)
            {
                move_lock = true;
            }

            draw_rectangle(player[0].x, player[0].y, player[0].w, player[0].h, ORANGE);
            for i in 1..player.len() {
                if i > 0 {
                    draw_rectangle(player[i].x, player[i].y, player[i].w, player[i].h, YELLOW);
                }
            }
            draw_rectangle(food_pos.x, food_pos.y, tile_size, tile_size, RED);

            if is_key_pressed(KeyCode::R) {
                food_pos = tile_position(tile_size, tiles_x, tiles_y);
            }

            draw_text(
                format!("Score: {player_score}").as_str(),
                20.,
                tiles_y * tile_size - 20.0,
                50.,
                WHITE,
            );

            draw_text(
                format!("Level: {level}").as_str(),
                tiles_x * tile_size - 170.,
                tiles_y * tile_size - 20.,
                42.,
                WHITE,
            );
        } else {
            draw_text("Game O'er", 300., 200., 50., WHITE);
            draw_text(
                format!("Final score is: {player_score}").as_str(),
                300.,
                300.,
                42.,
                WHITE,
            );
            if is_key_pressed(KeyCode::Enter) {
                player_score = 0;
                food_pos = tile_position(tile_size, tiles_x, tiles_y);
                level = set_level(player_score);
                speed = set_speed(level);

                player = Vec::new();

                player.push(Rect {
                    x: 0.,
                    y: 0.,
                    w: tile_size,
                    h: tile_size,
                });

                direction = Vec2 { x: 1., y: 0. };
                game_active = true;
            }
        }
        next_frame().await
    }
}

fn set_level(score: i32) -> i8 {
    match score {
        (0..5) => return 1,
        (5..10) => return 2,
        (10..15) => return 3,
        (15..20) => return 4,
        (20..25) => return 5,
        (25..30) => return 6,
        (30..40) => return 7,
        _ => return 8,
    }
    // return 1;
}

fn set_speed(level: i8) -> f32 {
    match level {
        1 => return 0.25,
        2 => return 0.2,
        3 => return 0.15,
        4 => return 0.12,
        5 => return 0.1,
        6 => return 0.08,
        7 => return 0.06,
        8 => return 0.04,
        _ => return 0.25,
    }
}

fn tile_position(tile_size: f32, tiles_x: f32, tiles_y: f32) -> Vec2 {
    return Vec2 {
        x: rand::gen_range(0, tiles_x as i32) as f32 * tile_size,
        y: rand::gen_range(0, tiles_y as i32) as f32 * tile_size,
    };
}

fn player_movement(dir: Vec2, move_lock: bool) -> Vec2 {
    // let mut move_lock = false;
    if is_key_pressed(KeyCode::Left) && dir.x != 1.0 && !move_lock {
        return Vec2 { x: -1.0, y: 0.0 };
    }
    if is_key_pressed(KeyCode::Right) && dir.x != -1.0 && !move_lock {
        return Vec2 { x: 1.0, y: 0.0 };
    }
    if is_key_pressed(KeyCode::Up) && dir.y != 1.0 && !move_lock {
        return Vec2 { x: 0.0, y: -1.0 };
    }
    if is_key_pressed(KeyCode::Down) && dir.y != -1.0 && !move_lock {
        return Vec2 { x: 0.0, y: 1.0 };
    }
    return dir;
}
