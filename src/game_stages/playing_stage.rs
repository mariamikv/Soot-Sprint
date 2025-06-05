use macroquad::prelude::*;
use crate::assets::Assets;
use crate::config;
use crate::types::{obstacle::Obstacle, obstacle_type::ObstacleType, score_type::ScoreType};
use super::{GameStage, StageTransition, PersistentData};
use super::game_over_stage::GameOverStage;

pub struct PlayingStage {
    player_y_position: f32,
    player_velocity_y: f32,
    is_on_floor: bool,
    obstacles: Vec<Obstacle>,
    spawn_timer: f32,
    scroll_speed: f32,
    score: u32,
    score_timer: f32,
}

impl PlayingStage {
    pub fn new(_screen_width: f32, _screen_height: f32) -> Self {
        Self {
            player_y_position: 100.0,
            player_velocity_y: 0.0,
            is_on_floor: false,
            obstacles: Vec::new(),
            spawn_timer: 2.0,
            scroll_speed: config::SCROLL_SPEED_INITIAL,
            score: 0,
            score_timer: 0.0,
        }
    }
}

impl GameStage for PlayingStage {
    fn update(
        &mut self,
        dt: f32,
        persistent_data: &mut PersistentData,
        screen_width: f32,
        _screen_height: f32,
    ) -> StageTransition {
        if is_key_pressed(KeyCode::Space) && self.is_on_floor {
            self.player_velocity_y = config::JUMP_FORCE;
        }
        self.player_velocity_y += config::GRAVITY;
        self.player_y_position += self.player_velocity_y;

        if self.player_y_position >= config::FLOOR_Y_POSITION {
            self.player_y_position = config::FLOOR_Y_POSITION;
            self.player_velocity_y = 0.0;
            self.is_on_floor = true;
        } else {
            self.is_on_floor = false;
        }

        self.score_timer += dt;
        if self.score_timer >= 0.1 {
            self.score += 1;
            self.score_timer = 0.0;
        }

        self.scroll_speed = match self.score {
            s if s >= ScoreType::FifthValue.value() => config::SCROLL_SPEED_TIER5,
            s if s >= ScoreType::FourthValue.value() => config::SCROLL_SPEED_TIER4,
            s if s >= ScoreType::ThirdValue.value() => config::SCROLL_SPEED_TIER3,
            s if s >= ScoreType::SecondValue.value() => config::SCROLL_SPEED_TIER2,
            s if s >= ScoreType::FirstValue.value() => config::SCROLL_SPEED_TIER1,
            _ => config::SCROLL_SPEED_INITIAL,
        };

        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 {
            self.spawn_timer = rand::gen_range(
                config::OBSTACLE_SPAWN_TIMER_MIN,
                config::OBSTACLE_SPAWN_TIMER_MAX,
            );
            let random_kind_index = rand::gen_range(0, 3);
            let new_obstacle = match random_kind_index {
                0 => Obstacle {
                    rect: Rect::new(
                        screen_width,
                        config::FLOOR_Y_POSITION - config::OBSTACLE_DEFAULT_HEIGHT,
                        config::OBSTACLE_DEFAULT_WIDTH,
                        config::OBSTACLE_DEFAULT_HEIGHT,
                    ),
                    kind: ObstacleType::Object0,
                },
                1 => {
                    let object1_rect_y = config::FLOOR_Y_POSITION - config::OBSTACLE_DEFAULT_HEIGHT
                        - config::OBJECT1_ADDITIONAL_CLEARANCE_ABOVE_OTHERS - config::OBJECT1_OWN_HEIGHT;
                    Obstacle {
                        rect: Rect::new(
                            screen_width,
                            object1_rect_y,
                            config::OBJECT1_OWN_WIDTH,
                            config::OBJECT1_OWN_HEIGHT,
                        ),
                        kind: ObstacleType::Object1,
                    }
                },
                _ => Obstacle {
                    rect: Rect::new(
                        screen_width,
                        config::FLOOR_Y_POSITION - config::OBSTACLE_DEFAULT_HEIGHT,
                        config::OBSTACLE_DEFAULT_WIDTH,
                        config::OBSTACLE_DEFAULT_HEIGHT,
                    ),
                    kind: ObstacleType::Object2,
                },
            };
            self.obstacles.push(new_obstacle);
        }

        for obstacle in self.obstacles.iter_mut() {
            obstacle.rect.x -= self.scroll_speed * dt;
        }
        self.obstacles.retain(|o| o.rect.x + o.rect.w > 0.0);

        let player_collision_rect = Rect::new(
            config::PLAYER_X_POSITION - config::PLAYER_RADIUS,
            self.player_y_position - config::PLAYER_RADIUS * 2.0,
            config::PLAYER_RADIUS * 2.0,
            config::PLAYER_RADIUS * 2.0,
        );

        for obstacle in &self.obstacles {
            if player_collision_rect.overlaps(&obstacle.rect) {
                persistent_data.high_score = persistent_data.high_score.max(self.score);
                return StageTransition::Switch(
                    Box::new(
                        GameOverStage::new(
                            self.score,
                            screen_width,
                            _screen_height,
                        )
                    )
                );
            }
        }

        StageTransition::None
    }

    fn draw(
        &self,
        assets: &Assets,
        _persistent_data: &PersistentData,
        screen_width: f32,
        screen_height: f32,
    ) {
        clear_background(WHITE);

        let bg_scale = 0.25;
        let bg_scaled_width = assets.background.width() * bg_scale;
        let bg_scaled_height = assets.background.height() * bg_scale;
        let bg_x_pos = (screen_width / 2.0) - (bg_scaled_width / 2.0);
        let bg_y_pos = screen_height - bg_scaled_height;
        draw_texture_ex(
            &assets.background,
            bg_x_pos,
            bg_y_pos,
            WHITE,
            DrawTextureParams {
                dest_size: Some(
                    Vec2::new(
                        bg_scaled_width,
                        bg_scaled_height,
                    )
                ),
                ..Default::default()
            }
        );

        draw_line(
            0.0,
            config::FLOOR_Y_POSITION,
            screen_width,
            config::FLOOR_Y_POSITION,
            5.0,
            BLACK,
        );

        let desired_player_collision_height = config::PLAYER_RADIUS * 2.0;
        let player_texture_height = assets.player.height();
        let player_display_scale = desired_player_collision_height / player_texture_height;
        let player_display_width = assets.player.width() * player_display_scale;

        let player_draw_x = config::PLAYER_X_POSITION - player_display_width / 2.0;
        let player_draw_y = self.player_y_position - desired_player_collision_height;

        draw_texture_ex(
            &assets.player,
            player_draw_x,
            player_draw_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(
                    Vec2::new(
                        player_display_width,
                        desired_player_collision_height,
                    )
                ),
                ..Default::default()
            },
        );

        for obstacle in &self.obstacles {
            let texture = match obstacle.kind {
                ObstacleType::Object0 => &assets.object0,
                ObstacleType::Object1 => &assets.object1,
                ObstacleType::Object2 => &assets.object2,
            };
            draw_texture_ex(
                texture,
                obstacle.rect.x,
                obstacle.rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(
                        Vec2::new(
                            obstacle.rect.w,
                            obstacle.rect.h,
                        )
                    ),
                    ..Default::default()
                },
            );
        }

        draw_text(
            "SOOT SPRINT",
            20.0,
            50.0,
            config::playing_ui::TITLE_FONT_SIZE,
            DARKGRAY,
        );
        let score_text = std::format!("SCORE: {}", self.score);
        let text_dimensions = measure_text(
            &score_text,
            None,
            config::playing_ui::SCORE_FONT_SIZE as u16,
            1.0,
        );
        let text_x = screen_width - text_dimensions.width - config::playing_ui::SCORE_MARGIN;
        let text_y = config::playing_ui::SCORE_MARGIN + config::playing_ui::SCORE_FONT_SIZE;
        draw_text(
            &score_text,
            text_x,
            text_y,
            config::playing_ui::SCORE_FONT_SIZE,
            BLACK,
        );
    }
}