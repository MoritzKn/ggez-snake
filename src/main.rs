#![feature(duration_extras)]

extern crate ggez;
extern crate rand;

mod apple;
mod assets;
mod base_types;
mod constants;
mod snake;
mod utils;

use apple::Apple;
use assets::Assets;
use base_types::*;
use constants::*;
use ggez::*;
use ggez::event::*;
use snake::Snake;
use std::time::Instant;
use utils::*;

struct MainState {
    snake: Snake,
    apple: Apple,
    assets: Assets,
    overlay_update_needed: bool,
    score_text: graphics::Text,
    game_over_text: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::load(ctx)?;

        let score_text = graphics::Text::new(ctx, "Score: 0", &assets.font_default)?;
        let game_over_text = graphics::Text::new(ctx, "Game Over", &assets.font_game_over)?;

        let state = MainState {
            snake: Snake::new(),
            apple: Apple::new(),
            assets,
            overlay_update_needed: true,
            score_text,
            game_over_text,
        };

        Ok(state)
    }

    fn update_overlay(&mut self, ctx: &mut Context) -> GameResult<()> {
        let score_text = format!("Score: {}", self.snake.score);
        self.score_text = graphics::Text::new(ctx, &score_text, &self.assets.font_default)?;

        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let snake = &mut self.snake;

        if snake.lost_at.is_none() && since(snake.last_round) > snake.speed {
            if let Some(ref input) = snake.input {
                snake.velocity = match *input {
                    Input::Up if snake.velocity.y != 1 => GridVector { x: 0, y: -1 },
                    Input::Down if snake.velocity.y != -1 => GridVector { x: 0, y: 1 },
                    Input::Left if snake.velocity.x != 1 => GridVector { x: -1, y: 0 },
                    Input::Right if snake.velocity.x != -1 => GridVector { x: 1, y: 0 },
                    _ => snake.velocity,
                };
            }
            snake.input = None;
            if snake.velocity != (GridVector { x: 0, y: 0 }) {
                let new_head;
                {
                    let len = snake.tail.len();
                    let last_head = &snake.tail[len - 1];
                    new_head = wrap_in_grid(GridVector {
                        x: last_head.x + snake.velocity.x,
                        y: last_head.y + snake.velocity.y,
                    });
                }

                for segment in &snake.tail {
                    if *segment == new_head {
                        snake.lost_at = Some(Instant::now());
                    }
                }

                if new_head == self.apple.position {
                    self.apple = Apple::new();
                    snake.speed -= snake.speed / SPEED_INCREASE_FRACTION;
                    snake.score += 1;
                    self.overlay_update_needed = true;
                    snake.grow = GROW_PER_APPLE;
                }

                if snake.grow > 0 {
                    snake.grow -= 1;
                } else {
                    snake.tail.remove(0);
                }
                snake.tail.push(new_head);
            }

            snake.last_round = Instant::now();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        {
            let apple = &self.apple;
            let since_spawn = since(apple.spawned_at);
            if since_spawn > APPLE_BLINK_TIME || blinks(since_spawn) {
                graphics::rectangle(ctx, graphics::DrawMode::Fill, gv_to_rect(&apple.position))?;
            }
        }

        {
            let snake = &self.snake;

            if snake.lost_at.is_none() {
                for gv in &snake.tail {
                    graphics::rectangle(ctx, graphics::DrawMode::Fill, gv_to_rect(gv))?;
                }
            }

            if let Some(lost_at) = snake.lost_at {
                let since_lost = since(lost_at);
                if since_lost < BLINK_LENGTH_AFTER_DEATH
                    && since_lost % (BLINK_INTERVAL * 2.0) < BLINK_INTERVAL
                {
                    for gv in &snake.tail {
                        graphics::rectangle(ctx, graphics::DrawMode::Fill, gv_to_rect(gv))?;
                    }
                }
            }
        }

        if self.overlay_update_needed {
            self.update_overlay(ctx)?;
            self.overlay_update_needed = false;
        }

        {
            let dest = graphics::Point2::new(10.0, 10.0);
            graphics::draw(ctx, &self.score_text, dest, 0.0)?;
        }

        if let Some(lost_at) = self.snake.lost_at {
            if since(lost_at) > GAME_OVER_TIMEOUT {
                let dest = graphics::Point2::new(100.0, 100.0);
                graphics::draw(ctx, &self.game_over_text, dest, 0.0)?;
            }
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W if self.snake.velocity.y != 1 => self.snake.input = Some(Input::Up),
            Keycode::S if self.snake.velocity.y != -1 => self.snake.input = Some(Input::Down),
            Keycode::A if self.snake.velocity.x != 1 => self.snake.input = Some(Input::Left),
            Keycode::D if self.snake.velocity.x != -1 => self.snake.input = Some(Input::Right),
            _ => {}
        }

        if keycode == Keycode::Escape {
            ctx.quit().unwrap();
        }
    }
}

pub fn main() {
    let cb = ContextBuilder::new("snake", "moritzkn")
        .window_setup(conf::WindowSetup::default().title("Snake"))
        .window_mode(conf::WindowMode::default().dimensions(
            GRID_SIZE.x as u32 * GRID_TILE_SIZE as u32,
            GRID_SIZE.y as u32 * GRID_TILE_SIZE as u32,
        ));

    let ctx = &mut cb.build().unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
