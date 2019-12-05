extern crate ggez;
extern crate rand;

mod apple;
mod assets;
mod base_types;
mod button;
mod constants;
mod snake;
mod stone;
mod utils;

use std::time::Instant;

use ggez::event::{quit, run, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{
    clear, draw, present, Align, DrawMode, Font, Mesh, Rect, Scale, StrokeOptions, Text,
};
use ggez::nalgebra as na;
use ggez::{conf, ContextBuilder};
use ggez::{Context, GameResult};

use apple::Apple;
use assets::Assets;
use base_types::*;
use button::Button;
use constants::*;
use snake::Snake;
use stone::{level_corners, Stone};
use utils::*;

const WINDOW_W: f32 = GRID_SIZE.x as f32 * GRID_TILE_SIZE;
const WINDOW_H: f32 = GRID_SIZE.y as f32 * GRID_TILE_SIZE + INFO_BAR_HIGHT;

fn create_restart_button(text: &str, font: Font) -> GameResult<Button> {
    Ok(Button::new(
        text,
        8.0 + FONT_DEFAULT_SIZE as f32,
        font,
        Rect::new(
            (WINDOW_W / 2.0) - (110.0 / 2.0),
            (WINDOW_H / 2.0) - (50.0 / 2.0) + 120.0,
            110.0,
            50.0,
        ),
    ))
}

struct MainState {
    snake: Snake,
    inputs: Vec<KeyCode>,
    apple: Apple,
    stones: Vec<Stone>,
    assets: Assets,
    ui_update_needed: bool,
    score_text: Text,
    game_over_text: Text,
    play_again: bool,
    restart_button: Option<Button>,
    game_over: bool,
}

fn spawn_apple_in_area(area: &GridArea, conflicts: &[GridVector]) -> Apple {
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
        let mut score_text = Text::new("Score: 0");
        score_text.set_font(assets.font, Scale::uniform(FONT_DEFAULT_SIZE));
        let mut game_over_text = Text::new("Game Over");
        game_over_text.set_font(assets.font, Scale::uniform(FONT_GAME_OVER_SIZE));

        let state = MainState {
            snake,
            inputs: vec![],
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

    fn reset(&mut self) {
        let snake = Snake::new();
        let apple = spawn_apple_in_area(&PLAY_AREA, &snake.tail);

        self.snake = snake;
        self.inputs = vec![];
        self.apple = apple;
        self.stones = level_corners();
        self.ui_update_needed = true;
        self.play_again = false;
        self.restart_button = None;
        self.game_over = false;
    }

    fn update_ui(&mut self) {
        let score_text = if self.game_over {
            format!("Final Score: {}", self.snake.score)
        } else {
            format!("Score: {}", self.snake.score)
        };
        self.score_text = Text::new(score_text);
        self.score_text
            .set_font(self.assets.font, Scale::uniform(FONT_DEFAULT_SIZE));
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update snake
        if !self.game_over {
            let snake = &mut self.snake;

            if let Some(lost_at) = snake.lost_at {
                self.game_over = since(lost_at) > GAME_OVER_TIMEOUT;
                self.ui_update_needed = true;
            } else if since(snake.last_round) > snake.speed {
                let mut new_velocity = None;
                for input in &self.inputs {
                    match *input {
                        KeyCode::W if snake.velocity.y != 1 => {
                            new_velocity = Some(GridVector { x: 0, y: -1 });
                        }
                        KeyCode::S if snake.velocity.y != -1 => {
                            new_velocity = Some(GridVector { x: 0, y: 1 });
                        }
                        KeyCode::A if snake.velocity.x != 1 => {
                            new_velocity = Some(GridVector { x: -1, y: 0 });
                        }
                        KeyCode::D if snake.velocity.x != -1 => {
                            new_velocity = Some(GridVector { x: 1, y: 0 });
                        }
                        _ => {}
                    }
                }

                self.inputs = vec![];

                if let Some(velocity) = new_velocity {
                    snake.velocity = velocity;
                }

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
                            Some(create_restart_button("Restart", self.assets.font)?);
                    }
                }
            }
        }

        if let Some(ref mut button) = self.restart_button {
            if button.unhandled_click() {
                self.play_again = true;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, COLOR_BACKGROUND);

        // Draw apple
        if !self.game_over {
            let apple = &self.apple;
            let since_spawn = since(apple.spawned_at);
            if since_spawn > APPLE_BLINK_TIME || blinks(since_spawn) {
                let rectangle = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    gv_to_rect(&apple.position),
                    COLOR_FOREGROUND,
                )?;
                draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
            }
        }

        // Draw snake
        {
            let snake = &self.snake;

            if snake.lost_at.is_none() {
                for segement in &snake.tail {
                    let rectangle = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        gv_to_rect(segement),
                        COLOR_FOREGROUND,
                    )?;
                    draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
                }
                let segement = &snake.tail[snake.tail.len() - 1];
                let rectangle = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    scale_rect(gv_to_rect(segement), -2.0),
                    COLOR_BACKGROUND,
                )?;
                draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
            } else if let Some(lost_at) = snake.lost_at {
                let since_lost = since(lost_at);
                if since_lost < BLINK_LENGTH_AFTER_DEATH
                    && since_lost % (BLINK_INTERVAL * 2.0) < BLINK_INTERVAL
                {
                    for segment in &snake.tail {
                        let rectangle = Mesh::new_rectangle(
                            ctx,
                            DrawMode::fill(),
                            gv_to_rect(segment),
                            COLOR_FOREGROUND,
                        )?;
                        draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
                    }
                }
            }
        }

        // Draw stones
        if !self.game_over {
            for stone in &self.stones {
                let rectangle = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    gv_to_rect(&stone.pos),
                    COLOR_FOREGROUND,
                )?;
                draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
            }
        }

        // Draw UI
        {
            if self.ui_update_needed {
                self.update_ui();
                self.ui_update_needed = false;
            }
            // Draw info bar
            if !self.game_over {
                // Draw background
                {
                    let width = WINDOW_W as f32;

                    let rect = Rect::new(0.0, 0.0, width, INFO_BAR_HIGHT);
                    let rectangle =
                        Mesh::new_rectangle(ctx, DrawMode::fill(), rect, COLOR_FOREGROUND)?;
                    draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
                    let rectangle = Mesh::new_rectangle(
                        ctx,
                        DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
                        scale_rect(rect, -2.0),
                        COLOR_BACKGROUND,
                    )?;
                    draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;
                }

                // Draw content
                {
                    let dest = na::Point2::new(
                        12.0,
                        (INFO_BAR_HIGHT - FONT_DEFAULT_SIZE as f32) / 2.0 - 4.0,
                    );
                    draw(ctx, &self.score_text, (dest,))?;
                }
            }

            if self.game_over {
                // Draw game over text
                let dest = na::Point2::new(
                    0.0,
                    (WINDOW_H / 2.0) - (FONT_GAME_OVER_SIZE as f32 * 1.4) / 2.0 - 60.0,
                );
                self.game_over_text
                    .set_bounds(na::Point2::new(WINDOW_W, WINDOW_H), Align::Center);
                draw(ctx, &self.game_over_text, (dest,))?;

                // Draw final score
                let dest = na::Point2::new(
                    0.0,
                    (WINDOW_H / 2.0) - (FONT_DEFAULT_SIZE as f32 * 1.4) / 2.0 + 10.0,
                );
                self.score_text
                    .set_bounds(na::Point2::new(WINDOW_W, WINDOW_H), Align::Center);
                draw(ctx, &self.score_text, (dest,))?;

                // Draw reset button
                if let Some(ref mut button) = self.restart_button {
                    button.draw(ctx)?;
                }
            }
        }

        if self.game_over && self.play_again {
            self.reset();
        }

        present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        self.inputs.push(keycode);

        if keycode == KeyCode::Space && self.restart_button.is_some() {
            self.play_again = true;
        }

        if keycode == KeyCode::Escape {
            quit(ctx);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        if let Some(index) = self.inputs.iter().position(|&i| i == keycode) {
            self.inputs.remove(index);
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, mb: MouseButton, x: f32, y: f32) {
        if let Some(ref mut button) = self.restart_button {
            button.notifiy_mouse_down(mb, x, y);
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, mb: MouseButton, x: f32, y: f32) {
        if let Some(ref mut button) = self.restart_button {
            button.notifiy_mouse_up(mb, x, y);
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        if let Some(ref mut button) = self.restart_button {
            button.notifiy_mouse_motion(x, y);
        }
    }
}

pub fn main() {
    let cb = ContextBuilder::new("snake", "moritzkn")
        .window_setup(conf::WindowSetup::default().title("Snake"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_W, WINDOW_H));

    let (ctx, event_loop) = &mut cb.build().unwrap();

    let state = &mut MainState::new(ctx).unwrap();
    run(ctx, event_loop, state).unwrap();
}
