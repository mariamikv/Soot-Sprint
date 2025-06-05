use macroquad::prelude::*;
use crate::assets::Assets;

#[derive(Debug, Default)]
pub struct PersistentData {
    pub high_score: u32,
}

impl PersistentData {
    pub fn new() -> Self {
        Default::default()
    }
}

pub enum StageTransition {
    None,
    Switch(Box<dyn GameStage>),
}

pub trait GameStage {

    fn update(
        &mut self,
        dt: f32,
        persistent_data: &mut PersistentData,
        screen_width: f32,
        screen_height: f32,
    ) -> StageTransition;

    fn draw(
        &self,
        assets: &Assets,
        persistent_data: &PersistentData,
        screen_width: f32,
        screen_height: f32,
    );
}

pub mod intro_stage;
pub mod playing_stage;
pub mod game_over_stage;