
// General Gameplay
pub const FLOOR_Y_POSITION: f32 = 350.0;
pub const PLAYER_X_POSITION: f32 = 75.0;
pub const GRAVITY: f32 = 1.0;
pub const JUMP_FORCE: f32 = -25.0;
pub const PLAYER_RADIUS: f32 = 40.0;

// Obstacles
pub const OBSTACLE_DEFAULT_WIDTH: f32 = 80.0;
pub const OBSTACLE_DEFAULT_HEIGHT: f32 = 60.0;
pub const OBJECT1_OWN_WIDTH: f32 = 70.0;
pub const OBJECT1_OWN_HEIGHT: f32 = 100.0;
pub const OBJECT1_ADDITIONAL_CLEARANCE_ABOVE_OTHERS: f32 = 30.0;
pub const OBSTACLE_SPAWN_TIMER_MIN: f32 = 1.5;
pub const OBSTACLE_SPAWN_TIMER_MAX: f32 = 3.0;

// Scroll Speed Tiers based on ScoreType values
pub const SCROLL_SPEED_INITIAL: f32 = 300.0;
pub const SCROLL_SPEED_TIER1: f32 = 400.0;
pub const SCROLL_SPEED_TIER2: f32 = 500.0;
pub const SCROLL_SPEED_TIER3: f32 = 600.0;
pub const SCROLL_SPEED_TIER4: f32 = 700.0;
pub const SCROLL_SPEED_TIER5: f32 = 800.0;

// UI and Intro
pub mod intro {
    pub const SLIDE_DURATION_S: f32 = 3.0;
    pub const BUTTON_WIDTH: f32 = 200.0;
    pub const BUTTON_HEIGHT: f32 = 50.0;
    pub const FONT_SIZE: f32 = 30.0;
    pub const LINE_HEIGHT_FACTOR: f32 = 1.3;
}

pub mod playing_ui {
    pub const SCORE_FONT_SIZE: f32 = 30.0;
    pub const SCORE_MARGIN: f32 = 20.0;
    pub const TITLE_FONT_SIZE: f32 = 50.0;
}

pub mod game_over_ui {
    pub const FONT_SIZE: f32 = 60.0;
    pub const SCORE_FONT_SIZE: f32 = 30.0;
}