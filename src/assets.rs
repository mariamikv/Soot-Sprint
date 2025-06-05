use macroquad::prelude::*;

pub struct Assets {
    pub background: Texture2D,
    pub player: Texture2D,
    pub object0: Texture2D,
    pub object1: Texture2D,
    pub object2: Texture2D,
    pub bold_font: Font,
}

impl Assets {
    pub async fn load() -> Result<Self, macroquad::Error> {
        let background = load_texture("assets/background_mid.png").await?;
        let player = load_texture("assets/player_0.png").await?;
        let object0 = load_texture("assets/object_0.png").await?;
        let object1 = load_texture("assets/object_1.png").await?;
        let object2 = load_texture("assets/object_2.png").await?;

        let bold_font = load_ttf_font("raw/bold.ttf").await?;
        Ok(
            Self {
                background,
                player,
                object0,
                object1,
                object2,
                bold_font,
            }
        )
    }
}