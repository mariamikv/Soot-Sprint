mod assets;
mod config;
mod game_stages;
mod state;
mod types;

use macroquad::prelude::*;
use assets::Assets;
use game_stages::{GameStage, StageTransition, PersistentData};
use game_stages::intro_stage::IntroStage;

#[macroquad::main("SootSprint")]
async fn main() {
    clear_background(BLACK);
    let loading_text = "Loading assets, please wait...";
    let text_params = TextParams {
        font_size: 40,
        font_scale: 1.0,
        color: WHITE,
        ..Default::default()
    };
    let text_dims = measure_text(
        loading_text, None,
        text_params.font_size,
        text_params.font_scale,
    );
    draw_text_ex(
        loading_text,
        screen_width() / 2.0 - text_dims.width / 2.0,
        screen_height() / 2.0,
        text_params,
    );

    next_frame().await;

    let assets = match Assets::load().await {
        Ok(loaded_assets) => loaded_assets,
        Err(e) => {
            error!("Failed to load assets: {}", e);
            loop {
                clear_background(RED);
                draw_text(
                    &std::format!("Error loading assets: {}", e),
                    20.0,
                    screen_height() / 2.0,
                    30.0,
                    BLACK,
                );
                next_frame().await;
            }
        }
    };

    let mut persistent_data = PersistentData::new();
    let mut current_stage: Box<dyn GameStage> = Box::new(
        IntroStage::new(
            screen_width(),
            screen_height(),
        )
    );

    loop {
        let dt = get_frame_time();
        let current_screen_width = screen_width();
        let current_screen_height = screen_height();

        let transition = current_stage.update(
            dt,
            &mut persistent_data,
            current_screen_width,
            current_screen_height,
        );

        match transition {
            StageTransition::Switch(new_stage_box) => {
                current_stage = new_stage_box;
            }
            StageTransition::None => {}
        }

        current_stage.draw(
            &assets,
            &persistent_data,
            current_screen_width,
            current_screen_height,
        );

        next_frame().await;
    }
}