use macroquad::math::Rect;
use crate::types::obstacle_type::ObstacleType;


pub struct Obstacle {
    pub rect: Rect,
    pub kind: ObstacleType,
}

impl Obstacle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, kind: ObstacleType) -> Self {
        Obstacle {
            rect: Rect::new(x, y, width, height),
            kind,
        }
    }
}