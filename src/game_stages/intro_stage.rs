use macroquad::prelude::*;
use crate::assets::Assets;
use crate::config;
use super::{GameStage, StageTransition, PersistentData};
use super::playing_stage::PlayingStage;

pub struct IntroStage {
    slides: Vec<String>,
    current_slide_index: usize,
    slide_timer: f32,
    start_button_rect: Rect,
    first_frame_passed: bool,
}

impl IntroStage {
    pub fn new(
        screen_width: f32,
        screen_height: f32,
    ) -> Self {
        Self {
            slides: vec![
                "Placeholder".to_string(),
                "In a dusty nook, where sunbeams dance no more, \n\
                 a tiny Soot Sprite flickers to life \n\
                 by a whisper of old magic.".to_string(),
                "Now, a grand adventure awaits! \n\
                 Hop and dash through a forgotten \n\
                 world of giant wonders".to_string(),
                "How far can your tiny feet carry you?".to_string(),
            ],
            current_slide_index: 0,
            slide_timer: 0.0,
            start_button_rect: Rect::new(
                screen_width * 0.4,
                screen_height * 0.7,
                config::intro::BUTTON_WIDTH,
                config::intro::BUTTON_HEIGHT,
            ),
            first_frame_passed: false,
        }
    }
}

impl GameStage for IntroStage {
    fn update(
        &mut self,
        dt: f32,
        _persistent_data: &mut PersistentData,
        screen_width: f32,
        screen_height: f32,
    ) -> StageTransition {
        self.start_button_rect.x = screen_width * 0.37;
        self.start_button_rect.y = screen_height * 0.5;
        self.start_button_rect.w = config::intro::BUTTON_WIDTH;
        self.start_button_rect.h = config::intro::BUTTON_HEIGHT;

        if self.current_slide_index == 0 && !self.first_frame_passed {
            self.first_frame_passed = true;
            self.slide_timer = 0.0;
            if self.slides.len() > 1 {
                self.current_slide_index += 1;
            }
        } else {
            self.slide_timer += dt;
            if self.slide_timer >= config::intro::SLIDE_DURATION_S {
                self.slide_timer = 0.0;
                if self.current_slide_index < self.slides.len() - 1 {
                    self.current_slide_index += 1;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if self.start_button_rect.contains(Vec2::new(mouse_x, mouse_y))
                && self.current_slide_index == self.slides.len() - 1 {
                return StageTransition::Switch(
                    Box::new(
                        PlayingStage::new(
                            screen_width,
                            screen_height,
                        )
                    )
                );
            }
        }
        StageTransition::None
    }

    fn draw(
        &self,
        _assets: &Assets,
        _persistent_data: &PersistentData,
        screen_width: f32,
        screen_height: f32,
    ) {
        clear_background(WHITE);
        if self.current_slide_index >= self.slides.len() {
            return;
        }

        let current_slide_full_text = &self.slides[self.current_slide_index];
        let lines: Vec<&str> = current_slide_full_text.lines().collect();

        let font_size = config::intro::FONT_SIZE;
        let line_spacing = font_size * config::intro::LINE_HEIGHT_FACTOR;
        let total_text_block_height = if lines.is_empty() {
            0.0
        } else {
            (lines.len() as f32 * line_spacing) - (line_spacing - font_size)
        };
        let mut current_y = screen_height * 0.4 - total_text_block_height / 2.0;

        for line_text in lines {
            let text_dims = measure_text(
                line_text,
                None,
                font_size as u16,
                1.0,
            );
            let start_x = screen_width / 2.0 - text_dims.width / 2.0;
            let text_params = TextParams {
                font: Some(&_assets.bold_font),
                font_size: font_size as u16,
                color: BLACK,
                ..TextParams::default()
            };

            draw_text_ex(
                line_text,
                start_x,
                current_y,
                text_params,
            );

            current_y += line_spacing;
        }

        if self.current_slide_index == self.slides.len() - 1 {
            draw_rectangle(
                self.start_button_rect.x,
                self.start_button_rect.y,
                self.start_button_rect.w,
                self.start_button_rect.h,
                BLACK,
            );
            let button_text = "Start Game";
            let button_font_size = config::intro::FONT_SIZE;
            let button_text_dims = measure_text(
                button_text,
                None,
                button_font_size as u16,
                1.0,
            );
            let text_params = TextParams {
                font: Some(&_assets.bold_font),
                color:WHITE,
                font_size: button_font_size as u16,
                ..TextParams::default()
            };
            draw_text_ex(
                button_text,
                self.start_button_rect.x + (self.start_button_rect.w - button_text_dims.width) / 2.0,
                self.start_button_rect.y + (self.start_button_rect.h - button_text_dims.height) / 2.0 + button_text_dims.offset_y,
                text_params,
            );
        }
    }
}