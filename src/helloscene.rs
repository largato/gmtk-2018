extern crate ggez;

use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::*;

use std::process;

use actor::*;
use scene::*;
use ship::*;

const VERTICAL_SPEED: f32 = 2.0;
const HORIZONTAL_SPEED: f32 = 3.0;

pub struct HelloScene {
    pub player: Ship,
}

impl Scene for HelloScene {
    fn update(&mut self, ctx: &mut Context) {
        self.player.update(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.player.draw(ctx);
    }

    fn on_key_down(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => self.player.pos.y -= VERTICAL_SPEED,
            Keycode::Down => self.player.pos.y += VERTICAL_SPEED,
            Keycode::Left => self.player.pos.x -= HORIZONTAL_SPEED,
            Keycode::Right => self.player.pos.x += HORIZONTAL_SPEED,
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
                self.player.pos.x += if value > 0 {
                    HORIZONTAL_SPEED
                } else {
                    -HORIZONTAL_SPEED
                }
            }
            Axis::LeftY => {
                self.player.pos.y += if value > 0 {
                    VERTICAL_SPEED
                } else {
                    -VERTICAL_SPEED
                }
            }
            _ => (),
}
    }
}
