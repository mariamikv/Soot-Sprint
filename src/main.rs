use macroquad::prelude::*;

#[derive(Clone, Copy)] // Add this so we can copy the value
enum ObstacleType {
    Object0,
    Object1,
    Object2,
}

struct Obstacle {
    rect: Rect,
    kind: ObstacleType,
}

enum GameState {
    Playing,
    GameOver,
}

#[macroquad::main("SootSprint")]
async fn main() {
    let background_texture = load_texture("assets/background_mid.png").await.unwrap();
    let player_texture = load_texture("assets/first_jump.png").await.unwrap();

    let obstacle_object0_texture = load_texture("assets/object_0.png").await.unwrap();
    let obstacle_object1_texture = load_texture("assets/object_1.png").await.unwrap();
    let obstacle_object2_texture = load_texture("assets/object_2.png").await.unwrap();

    const FLOOR_Y_POSITION: f32 = 350.0;
    const PLAYER_X_POSITION: f32 = 75.0;
    const GRAVITY: f32 = 1.0;
    const JUMP_FORCE: f32 = -25.0;

    const OBSTACLE_WIDTH: f32 = 60.0;
    const OBSTACLE_HEIGHT: f32 = 60.0;

    let mut player_y_position = 100.0;
    let mut player_velocity_y = 0.0;
    let player_radius = 40.0;
    let mut is_on_floor = false;

    let mut obstacles: Vec<Obstacle> = vec![];
    let mut spawn_timer = 2.0;
    let scroll_speed = 300.0;

    let mut game_state = GameState::Playing;

    let mut score = 0;
    let mut score_timer = 0.0;

    loop {
        let delta_time = get_frame_time();

        match game_state {
            GameState::Playing => {
                if is_key_pressed(KeyCode::Space) && is_on_floor {
                    player_velocity_y = JUMP_FORCE;
                }
                player_velocity_y += GRAVITY;
                player_y_position += player_velocity_y;
                if player_y_position >= FLOOR_Y_POSITION {
                    player_y_position = FLOOR_Y_POSITION;
                    player_velocity_y = 0.0;
                    is_on_floor = true;
                } else {
                    is_on_floor = false;
                }
                spawn_timer -= delta_time;

                score_timer += delta_time;
                if score_timer >= 0.1 {
                    score +=1;
                    score_timer = 0.0;
                }

                if spawn_timer <= 0.0 {
                    spawn_timer = rand::gen_range(1.5, 3.0);

                    let random_kind_index = rand::gen_range(0, 3);
                    let new_obstacle = match random_kind_index {
                        0 => Obstacle {
                            rect: Rect::new(screen_width(), FLOOR_Y_POSITION - OBSTACLE_HEIGHT, OBSTACLE_WIDTH, OBSTACLE_HEIGHT),
                            kind: ObstacleType::Object0,
                        },
                        1 => Obstacle {
                            rect: Rect::new(screen_width(), FLOOR_Y_POSITION - OBSTACLE_HEIGHT, OBSTACLE_WIDTH, OBSTACLE_HEIGHT),
                            kind: ObstacleType::Object1,
                        },
                        _ => Obstacle {
                            rect: Rect::new(screen_width(), FLOOR_Y_POSITION - OBSTACLE_HEIGHT, OBSTACLE_WIDTH, OBSTACLE_HEIGHT),
                            kind: ObstacleType::Object2,
                        }
                    };
                    obstacles.push(new_obstacle);
                }
                for obstacle in obstacles.iter_mut() {
                    obstacle.rect.x -= scroll_speed * delta_time;
                }
                obstacles.retain(|o| o.rect.x + o.rect.w > 0.0);
                let player_rect = Rect::new(
                    PLAYER_X_POSITION - player_radius,
                    player_y_position - player_radius * 2.0,
                    player_radius * 2.0,
                    player_radius * 2.0,
                );
                for obstacle in &obstacles {
                    if player_rect.overlaps(&obstacle.rect) {
                        game_state = GameState::GameOver;
                    }
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    player_y_position = 100.0;
                    player_velocity_y = 0.0;
                    obstacles.clear();
                    spawn_timer = 2.0;
                    score = 0;
                    game_state = GameState::Playing;
                }
            }
        }

        clear_background(WHITE);

        let scale = 0.25;

        let scaled_width = background_texture.width() * scale;
        let scaled_height = background_texture.height() * scale;

        let bg_x_pos = (screen_width() / 2.0) - (scaled_width / 2.0);
        let bg_y_pos = screen_height() - scaled_height;

        draw_texture_ex(
            &background_texture,
            bg_x_pos,
            bg_y_pos,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(scaled_width, scaled_height)),
                ..Default::default()
            },
        );

        draw_line(0.0, FLOOR_Y_POSITION, screen_width(), FLOOR_Y_POSITION, 5.0, BLACK);

        let desired_player_radius = player_radius;
        let texture_width = player_texture.width();
        let texture_height = player_texture.height();

        let scale = desired_player_radius * 2.0 / texture_width;

        let scaled_width = texture_width * scale;
        let scaled_height = texture_height * scale;

        let player_draw_x = PLAYER_X_POSITION - scaled_width / 2.0;
        let player_draw_y = player_y_position - scaled_height;

        draw_texture_ex(
            &player_texture,
            player_draw_x,
            player_draw_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(scaled_width, scaled_height)),
                ..Default::default()
            },
        );

        for obstacle in &obstacles {
            let texture = match obstacle.kind {
                ObstacleType::Object0 => &obstacle_object0_texture,
                ObstacleType::Object1 => &obstacle_object1_texture,
                ObstacleType::Object2 => &obstacle_object2_texture,
            };

            draw_texture_ex(
                texture,
                obstacle.rect.x,
                obstacle.rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(obstacle.rect.w, obstacle.rect.h)),
                    ..Default::default()
                },
            );
        }

        draw_text("SOOT SPRINT", 20.0, 50.0, 50.0, DARKGRAY);

        let score_text = format!("SCORE: {}", score);
        let font_size = 30.0;
        let text_color = BLACK;
        let margin = 20.0;

        let text_dimensions = measure_text(&score_text, None, font_size as u16, 1.0);

        let text_x = screen_width() - text_dimensions.width - margin;
        let text_y = margin + font_size;

        draw_text(&score_text, text_x, text_y, font_size, text_color);

        if let GameState::GameOver = game_state {
            let text = "GAME OVER!";
            let text_dims = measure_text(text, None, 60, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dims.width / 2.0,
                screen_height() / 2.0,
                60.0,
                BLACK,
            );
        }

        next_frame().await
    }
}