extern crate ggez;

use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::graphics::{DrawMode, Rect};
use ggez::*;
use std::process;

const VERTICAL_SPEED: f32 = 2.0;
const HORIZONTAL_SPEED: f32 = 3.0;

struct MainState {
    pos_x: f32,
    pos_y: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            pos_y: 0.0,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            Rect::new(self.pos_x, self.pos_y, 20.0, 20.0),
        )?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        match keycode {
            Keycode::Up => self.pos_y -= VERTICAL_SPEED,
            Keycode::Down => self.pos_y += VERTICAL_SPEED,
            Keycode::Left => self.pos_x -= HORIZONTAL_SPEED,
            Keycode::Right => self.pos_x += HORIZONTAL_SPEED,
            Keycode::Escape => process::exit(0),
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
    }

    fn controller_button_down_event(&mut self, _ctx: &mut Context, btn: Button, _ctrl_id: i32) {
        match btn {
            Button::A => (), // Shoot! pew pew!
            _ => (),
        }
    }

    fn controller_button_up_event(&mut self, _ctx: &mut Context, _btn: Button, _ctrl_id: i32) {}

    fn controller_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: i16, _ctrl_id: i32) {
        match axis {
            Axis::LeftX => {
                self.pos_x += if value > 0 {
                    HORIZONTAL_SPEED
                } else {
                    -HORIZONTAL_SPEED
                }
            }
            Axis::LeftY => {
                self.pos_y += if value > 0 {
                    VERTICAL_SPEED
                } else {
                    -VERTICAL_SPEED
                }
            }
            _ => (),
        }
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            println!("Focus gained");
        } else {
            println!("Focus lost");
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
