use macroquad::prelude::*;
use crate::assets::Assets;
use crate::config;
use super::{GameStage, StageTransition, PersistentData};
use super::playing_stage::PlayingStage;

pub struct GameOverStage {
    final_score: u32,
}

impl GameOverStage {
    pub fn new(
        score: u32,
        _screen_width: f32,
        _screen_height: f32,
    ) -> Self {
        Self { final_score: score }
    }
}

impl GameStage for GameOverStage {
    fn update(
        &mut self,
        _dt: f32,
        _persistent_data: &mut PersistentData,
        screen_width: f32,
        screen_height: f32,
    ) -> StageTransition {
        if is_key_pressed(KeyCode::Space) {
            return StageTransition::Switch(
                Box::new(
                    PlayingStage::new(
                        screen_width,
                        screen_height,
                    )
                )
            );
        }
        StageTransition::None
    }

    fn draw(
        &self,
        _assets: &Assets,
        persistent_data: &PersistentData,
        screen_width: f32,
        screen_height: f32,
    ) {
        clear_background(WHITE);

        let game_over_text = "GAME OVER!";
        let text_dims_game_over = measure_text(
            game_over_text, None,
            config::game_over_ui::FONT_SIZE as u16,
            1.0,
        );
        draw_text(
            game_over_text,
            screen_width / 2.0 - text_dims_game_over.width / 2.0,
            screen_height / 2.0,
            config::game_over_ui::FONT_SIZE,
            BLACK,
        );

        let final_score_text = std::format!("Your Score: {}", self.final_score);
        let final_score_dims = measure_text(
            &final_score_text,
            None,
            config::game_over_ui::SCORE_FONT_SIZE as u16,
            1.0,
        );
        draw_text(
            &final_score_text,
            screen_width / 2.0 - final_score_dims.width / 2.0,
            screen_height / 2.0 + config::game_over_ui::FONT_SIZE + 10.0,
            config::game_over_ui::SCORE_FONT_SIZE,
            DARKGRAY,
        );

        let high_score_text = std::format!("High Score: {}", persistent_data.high_score);
        let high_score_dims = measure_text(
            &high_score_text,
            None,
            config::game_over_ui::SCORE_FONT_SIZE as u16,
            1.0,
        );
        draw_text(
            &high_score_text,
            screen_width / 2.0 - high_score_dims.width / 2.0,
            screen_height / 2.0 + config::game_over_ui::FONT_SIZE + 10.0 + config::game_over_ui::SCORE_FONT_SIZE + 10.0,
            config::game_over_ui::SCORE_FONT_SIZE,
            DARKGRAY,
        );

        let restart_prompt_text = "Press SPACE to Restart";
        let restart_prompt_dims = measure_text(
            restart_prompt_text,
            None,
            config::game_over_ui::SCORE_FONT_SIZE as u16,
            1.0,
        );
        draw_text(
            restart_prompt_text,
            screen_width / 2.0 - restart_prompt_dims.width / 2.0,
            screen_height * 0.8,
            config::game_over_ui::SCORE_FONT_SIZE,
            BLACK,
        );
    }
}