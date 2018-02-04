use constants::*;
use ggez::*;

pub struct Assets {
    pub font_default: graphics::Font,
    pub font_game_over: graphics::Font,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> GameResult<Self> {
        let font_default = graphics::Font::new(ctx, "/VT323-Regular.ttf", FONT_DEFAULT_SIZE)?;
        let font_game_over = graphics::Font::new(ctx, "/VT323-Regular.ttf", FONT_GAME_OVER_SIZE)?;
        Ok(Assets {
            font_default,
            font_game_over,
        })
    }
}
