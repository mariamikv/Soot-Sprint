use macroquad::prelude::*;

struct Obstacle {
    rect: Rect,
}

enum GameState {
    Playing,
    GameOver,
}

#[macroquad::main("SootSprint")]
async fn main() {
    let background_texture = load_texture("assets/background_mid.png").await.unwrap();
    let player_texture = load_texture("assets/first_jump.png").await.unwrap();

    const FLOOR_Y_POSITION: f32 = 400.0;
    const PLAYER_X_POSITION: f32 = 75.0;
    const GRAVITY: f32 = 1.0;
    const JUMP_FORCE: f32 = -25.0;

    let mut player_y_position = 100.0;
    let mut player_velocity_y = 0.0;
    let player_radius = 20.0;
    let mut is_on_floor = false;

    let mut obstacles: Vec<Obstacle> = vec![];
    let mut spawn_timer = 2.0;
    let scroll_speed = 200.0;

    let mut game_state = GameState::Playing;

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
                if spawn_timer <= 0.0 {
                    spawn_timer = rand::gen_range(1.5, 3.0);
                    obstacles.push(Obstacle {
                        rect: Rect::new(screen_width(), FLOOR_Y_POSITION - 80.0, 30.0, 80.0),
                    });
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
        // --- NEW: Use draw_texture_ex to scale the player image ---
        let desired_player_radius = player_radius; // The size our circle was
        let texture_width = player_texture.width() as f32;
        let texture_height = player_texture.height() as f32;

        // Calculate the scale needed to make the image roughly the size of the circle
        let scale = desired_player_radius * 2.0 / texture_width; // Assuming width is the dominant dimension

        let scaled_width = texture_width * scale;
        let scaled_height = texture_height * scale;

        // Calculate the position to center the scaled image
        let player_draw_x = PLAYER_X_POSITION - scaled_width / 2.0;
        let player_draw_y = player_y_position - scaled_height; // Align bottom

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
            draw_rectangle(obstacle.rect.x, obstacle.rect.y, obstacle.rect.w, obstacle.rect.h, BROWN);
        }

        draw_text("SOOT SPRINT", 20.0, 50.0, 50.0, DARKGRAY);

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