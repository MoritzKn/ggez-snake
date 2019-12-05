use ggez::event::MouseButton;
use ggez::graphics::{
    draw, Align, DrawMode, Font, Mesh, Rect, Scale, StrokeOptions, Text, TextFragment,
};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use constants::*;
use utils::*;

fn pos_in_rect(rect: Rect, x: f32, y: f32) -> bool {
    x > rect.x && y > rect.y && x < rect.x + rect.w && y < rect.y + rect.h
}

pub struct Button {
    pub text: String,
    pub font: Font,
    pub line_hight: f32,
    pub rect: Rect,
    pub under_mouse: bool,
    pub pressed: bool,
    pub was_clicked: bool,
}

impl Button {
    pub fn new(text: &str, line_hight: f32, font: Font, rect: Rect) -> Self {
        Button {
            text: text.to_string(),
            font,
            line_hight,
            rect,
            under_mouse: false,
            pressed: false,
            was_clicked: false,
        }
    }

    pub fn notifiy_mouse_down(&mut self, mb: MouseButton, x: f32, y: f32) {
        if mb == MouseButton::Left {
            self.under_mouse = pos_in_rect(self.rect, x, y);
            if self.under_mouse {
                self.pressed = true;
            } else {
                self.pressed = false;
            }
        }
    }

    pub fn notifiy_mouse_up(&mut self, mb: MouseButton, x: f32, y: f32) {
        if mb == MouseButton::Left {
            self.under_mouse = pos_in_rect(self.rect, x, y);
            if self.under_mouse && self.pressed {
                self.was_clicked = true;
            } else {
                self.pressed = false;
            }
        }
    }

    pub fn notifiy_mouse_motion(&mut self, x: f32, y: f32) {
        self.under_mouse = pos_in_rect(self.rect, x, y);
    }

    pub fn unhandled_click(&mut self) -> bool {
        let value = self.was_clicked;
        self.was_clicked = false;
        value
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.under_mouse {
            let rectangle =
                Mesh::new_rectangle(ctx, DrawMode::fill(), self.rect, COLOR_FOREGROUND)?;
            draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;

            let rectangle = Mesh::new_rectangle(
                ctx,
                DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
                scale_rect(self.rect, -2.0),
                COLOR_BACKGROUND,
            )?;
            draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
        } else {
            let rectangle = Mesh::new_rectangle(
                ctx,
                DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
                scale_rect(self.rect, -1.0),
                COLOR_FOREGROUND,
            )?;
            draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
        }

        {
            let dest = na::Point2::new(
                self.rect.x,
                self.rect.y + self.rect.h / 2.0 - self.line_hight / 2.0,
            );

            let frag = TextFragment::new(self.text.to_string());
            let mut text = if self.under_mouse {
                Text::new(frag.color(COLOR_BACKGROUND))
            } else {
                Text::new(frag)
            };

            text.set_font(self.font, Scale::uniform(FONT_DEFAULT_SIZE));
            text.set_bounds(na::Point2::new(self.rect.w, self.rect.h), Align::Center);

            draw(ctx, &text, (dest,))?;
        }

        Ok(())
    }
}
