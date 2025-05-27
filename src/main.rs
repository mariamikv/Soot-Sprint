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
    let mut scroll_speed = 200.0;

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

        clear_background(color_u8!(217, 217, 217, 255));

        draw_line(0.0, FLOOR_Y_POSITION, screen_width(), FLOOR_Y_POSITION, 5.0, BLACK);
        draw_circle(PLAYER_X_POSITION, player_y_position - player_radius, player_radius, BLACK);

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