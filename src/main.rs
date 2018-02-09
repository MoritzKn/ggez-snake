extern crate ggez;
extern crate rand;

mod apple;
mod assets;
mod base_types;
mod stone;
mod button;
mod constants;
mod snake;
mod utils;

use apple::Apple;
use assets::Assets;
use base_types::*;
use button::Button;
use constants::*;
use ggez::*;
use ggez::event::*;
use snake::Snake;
use std::time::Instant;
use stone::{level_corners, Stone};
use utils::*;

fn create_restart_button(ctx: &mut Context, font: &graphics::Font) -> GameResult<Button> {
    Ok(Button::new(
        graphics::Text::new(ctx, "Restart", font)?,
        8.0 + FONT_DEFAULT_SIZE as f32,
        75.0,
        graphics::Rect::new(
            (ctx.conf.window_mode.width as f32 / 2.0) - (110.0 / 2.0),
            (ctx.conf.window_mode.height as f32 / 2.0) - (50.0 / 2.0) + 120.0,
            110.0,
            50.0,
        ),
    ))
}

struct MainState {
    snake: Snake,
    apple: Apple,
    stones: Vec<Stone>,
    assets: Assets,
    ui_update_needed: bool,
    score_text: graphics::Text,
    game_over_text: graphics::Text,
    play_again: bool,
    restart_button: Option<Button>,
    game_over: bool,
}

fn spawn_apple_in_area(area: &GridArea, conflicts: &Vec<GridVector>) -> Apple {
    let pos = loop {
        let pos = random_pos(area);

        if conflicts
            .iter()
            .all(|conflicting_pos| *conflicting_pos != pos)
        {
            break pos;
        }
    };

    Apple::new(pos)
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let snake = Snake::new();
        let apple = spawn_apple_in_area(&PLAY_AREA, &snake.tail);
        let assets = Assets::load(ctx)?;
        let score_text = graphics::Text::new(ctx, "Score: 0", &assets.font_default)?;
        let game_over_text = graphics::Text::new(ctx, "Game Over", &assets.font_game_over)?;

        let state = MainState {
            snake,
            apple,
            stones: level_corners(),
            assets,
            ui_update_needed: true,
            score_text,
            game_over_text,
            play_again: false,
            restart_button: None,
            game_over: false,
        };

        Ok(state)
    }

    fn update_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        let score_text = if self.game_over {
            format!("Final Score: {}", self.snake.score)
        } else {
            format!("Score: {}", self.snake.score)
        };
        self.score_text = graphics::Text::new(ctx, &score_text, &self.assets.font_default)?;

        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update snake
        if !self.game_over {
            let snake = &mut self.snake;

            if let Some(lost_at) = snake.lost_at {
                self.game_over = since(lost_at) > GAME_OVER_TIMEOUT;
                self.ui_update_needed = true;
            } else if since(snake.last_round) > snake.speed {
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

                    for stone in &self.stones {
                        if stone.pos == new_head {
                            snake.lost_at = Some(Instant::now());
                        }
                    }

                    if new_head == self.apple.position {
                        self.apple = spawn_apple_in_area(&PLAY_AREA, &snake.tail);
                        snake.speed -= snake.speed / SPEED_INCREASE_FRACTION;
                        snake.score += 1;
                        self.ui_update_needed = true;
                        snake.grow += GROW_PER_APPLE;
                    }

                    if snake.lost_at.is_none() {
                        if snake.grow > 0 {
                            snake.grow -= 1;
                        } else {
                            snake.tail.remove(0);
                        }
                        snake.tail.push(new_head);
                    }
                }

                snake.last_round = Instant::now();
            }
        }

        // Add restart button
        {
            if self.restart_button.is_none() {
                if let Some(lost_at) = self.snake.lost_at {
                    if since(lost_at) > GAME_OVER_TIMEOUT {
                        self.restart_button =
                            Some(create_restart_button(ctx, &self.assets.font_default)?);
                    }
                }
            }
        }

        if let Some(ref mut button) = self.restart_button {
            if button.unhandled_click() {
                self.play_again = true;
                ctx.quit()?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, COLOR_BACKGROUND);
        graphics::set_color(ctx, COLOR_FOREGROUND)?;
        graphics::clear(ctx);

        // Draw apple
        if !self.game_over {
            let apple = &self.apple;
            let since_spawn = since(apple.spawned_at);
            if since_spawn > APPLE_BLINK_TIME || blinks(since_spawn) {
                graphics::rectangle(ctx, graphics::DrawMode::Fill, gv_to_rect(&apple.position))?;
            }
        }

        // Draw snake
        {
            let snake = &self.snake;

            if snake.lost_at.is_none() {
                for segement in &snake.tail {
                    graphics::rectangle(ctx, graphics::DrawMode::Fill, gv_to_rect(segement))?;
                }
                let segement = &snake.tail[snake.tail.len() - 1];
                graphics::set_color(ctx, COLOR_BACKGROUND)?;
                graphics::rectangle(
                    ctx,
                    graphics::DrawMode::Fill,
                    scale_rect(gv_to_rect(segement), -2.0),
                )?;
                graphics::set_color(ctx, COLOR_FOREGROUND)?;
            } else if let Some(lost_at) = snake.lost_at {
                let since_lost = since(lost_at);
                if since_lost < BLINK_LENGTH_AFTER_DEATH
                    && since_lost % (BLINK_INTERVAL * 2.0) < BLINK_INTERVAL
                {
                    for segment in &snake.tail {
                        graphics::rectangle(ctx, graphics::DrawMode::Fill, gv_to_rect(segment))?;
                    }
                }
            }
        }

        // Draw stones
        if !self.game_over {
            for stone in &self.stones {
                graphics::rectangle(ctx, graphics::DrawMode::Fill, gv_to_rect(&stone.pos))?;
            }
        }

        // Draw UI
        {
            if self.ui_update_needed {
                self.update_ui(ctx)?;
                self.ui_update_needed = false;
            }
            // Draw info bar
            if !self.game_over {
                // Draw background
                {
                    let width = ctx.conf.window_mode.width as f32;

                    let rect = graphics::Rect::new(0.0, 0.0, width, INFO_BAR_HIGHT);
                    graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;

                    graphics::set_color(ctx, COLOR_BACKGROUND)?;
                    let rect = scale_rect(rect, -2.0);
                    graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), rect)?;
                    graphics::set_color(ctx, COLOR_FOREGROUND)?;
                }

                // Draw content
                {
                    graphics::set_color(ctx, COLOR_BACKGROUND)?;

                    let dest = graphics::Point2::new(
                        12.0,
                        (INFO_BAR_HIGHT - FONT_DEFAULT_SIZE as f32) / 2.0 - 4.0,
                    );
                    graphics::draw(ctx, &self.score_text, dest, 0.0)?;

                    graphics::set_color(ctx, COLOR_FOREGROUND)?;
                }
            }

            if self.game_over {
                // Draw game over text
                let center = graphics::Point2::new(
                    ctx.conf.window_mode.width as f32 / 2.0,
                    ctx.conf.window_mode.height as f32 / 2.0,
                );
                let dest = graphics::Point2::new(
                    center.x - 180.0,
                    center.y - (FONT_GAME_OVER_SIZE as f32 * 1.4) / 2.0 - 60.0,
                );
                graphics::draw(ctx, &self.game_over_text, dest, 0.0)?;

                // Draw final score
                let dest = graphics::Point2::new(
                    center.x - 84.0,
                    center.y - (FONT_DEFAULT_SIZE as f32 * 1.4) / 2.0 + 10.0,
                );
                graphics::draw(ctx, &self.score_text, dest, 0.0)?;

                // Draw reset button
                if let Some(ref mut button) = self.restart_button {
                    button.draw(ctx)?;
                }
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
            Keycode::Space if self.restart_button.is_some() => {
                self.play_again = true;
                ctx.quit().unwrap();
            }
            _ => {}
        }

        if keycode == Keycode::Escape {
            ctx.quit().unwrap();
        }
    }
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, mb: MouseButton, x: i32, y: i32) {
        if let Some(ref mut button) = self.restart_button {
            button.notifiy_mouse_down(mb, x, y);
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, mb: MouseButton, x: i32, y: i32) {
        if let Some(ref mut button) = self.restart_button {
            button.notifiy_mouse_up(mb, x, y);
        }
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        if let Some(ref mut button) = self.restart_button {
            button.notifiy_mouse_motion(x, y);
        }
    }
}

pub fn main() {
    let cb = ContextBuilder::new("snake", "moritzkn")
        .window_setup(conf::WindowSetup::default().title("Snake"))
        .window_mode(conf::WindowMode::default().dimensions(
            GRID_SIZE.x as u32 * GRID_TILE_SIZE as u32,
            GRID_SIZE.y as u32 * GRID_TILE_SIZE as u32 + INFO_BAR_HIGHT as u32,
        ));

    let ctx = &mut cb.build().unwrap();

    loop {
        let state = &mut MainState::new(ctx).unwrap();
        event::run(ctx, state).unwrap();

        if !state.play_again {
            break;
        }
    }
}
