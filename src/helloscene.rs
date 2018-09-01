extern crate ggez;

use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::graphics::{DrawMode, Rect};
use ggez::*;

use std::process;

use scene::*;

const VERTICAL_SPEED: f32 = 2.0;
const HORIZONTAL_SPEED: f32 = 3.0;

pub struct HelloScene {
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Scene for HelloScene {
    fn update(&mut self, _ctx: &mut Context) {
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::rectangle(
            ctx,
            DrawMode::Fill,
            Rect::new(self.pos_x, self.pos_y, 20.0, 20.0),
        ).unwrap();
    }

    fn on_key_down(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => self.pos_y -= VERTICAL_SPEED,
            Keycode::Down => self.pos_y += VERTICAL_SPEED,
            Keycode::Left => self.pos_x -= HORIZONTAL_SPEED,
            Keycode::Right => self.pos_x += HORIZONTAL_SPEED,
            Keycode::Escape => process::exit(0),
            _ => (),
        }
    }

    fn on_controller_button_down(&mut self, _ctx: &mut Context, btn: Button, _ctrl_id: i32) {
        match btn {
            Button::A => (), // Shoot! pew pew!
            _ => (),
        }
    }

    fn on_controller_axis(&mut self, _ctx: &mut Context, axis: Axis, value: i16, _ctrl_id: i32) {
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
}
