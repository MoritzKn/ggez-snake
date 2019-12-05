use ggez::*;

pub struct Assets {
    pub font: graphics::Font,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> GameResult<Self> {
        let font = graphics::Font::new(ctx, "/SHPinscher-Regular.ttf")?;
        Ok(Assets { font })
    }
}
