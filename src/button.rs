use constants::*;
use ggez::event::MouseButton;
use ggez::*;
use utils::*;

fn pos_in_rect(rect: graphics::Rect, x: i32, y: i32) -> bool {
    x > rect.x as i32 && y > rect.y as i32 && x < rect.x as i32 + rect.w as i32
        && y < rect.y as i32 + rect.h as i32
}

pub struct Button {
    pub text: graphics::Text,
    pub line_hight: f32,
    pub text_length: f32,
    pub rect: graphics::Rect,
    pub under_mouse: bool,
    pub pressed: bool,
    pub was_clicked: bool,
}

impl Button {
    pub fn new(
        text: graphics::Text,
        line_hight: f32,
        text_length: f32,
        rect: graphics::Rect,
    ) -> Self {
        Button {
            text,
            line_hight,
            text_length,
            rect,
            under_mouse: false,
            pressed: false,
            was_clicked: false,
        }
    }

    pub fn notifiy_mouse_down(&mut self, mb: MouseButton, x: i32, y: i32) {
        if mb == MouseButton::Left {
            self.under_mouse = pos_in_rect(self.rect, x, y);
            if self.under_mouse {
                self.pressed = true;
            } else {
                self.pressed = false;
            }
        }
    }

    pub fn notifiy_mouse_up(&mut self, mb: MouseButton, x: i32, y: i32) {
        if mb == MouseButton::Left {
            self.under_mouse = pos_in_rect(self.rect, x, y);
            if self.under_mouse && self.pressed {
                println!("click");
                self.was_clicked = true;
            } else {
                self.pressed = false;
            }
        }
    }

    pub fn notifiy_mouse_motion(&mut self, x: i32, y: i32) {
        self.under_mouse = pos_in_rect(self.rect, x, y);
    }

    pub fn unhandled_click(&mut self) -> bool {
        let value = self.was_clicked;
        self.was_clicked = false;
        value
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.under_mouse {
            graphics::rectangle(ctx, graphics::DrawMode::Fill, self.rect)?;

            graphics::set_color(ctx, COLOR_BACKGROUND)?;
            let rect = scale_rect(self.rect, -2.0);
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), rect)?;
        } else {
            let rect = scale_rect(self.rect, -1.0);
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), rect)?;
        }

        {
            let dest = graphics::Point2::new(
                self.rect.x + self.rect.w / 2.0 - self.text_length / 2.0,
                self.rect.y + self.rect.h / 2.0 - self.line_hight / 2.0,
            );
            graphics::draw(ctx, &self.text, dest, 0.0)?;
        }

        graphics::set_color(ctx, COLOR_FOREGROUND)?;
        Ok(())
    }
}
