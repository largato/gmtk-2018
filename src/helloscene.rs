extern crate ggez;

use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::*;

use std::process;

use actor::*;
use scene::*;
use ship::*;

pub struct HelloScene {
    pub player: Ship,

    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
}

impl HelloScene {
    fn update_player_pos(&mut self) {
        if self.left_pressed {
            self.player.pos.x -= self.player.horizontal_speed;
        } else if self.right_pressed {
            self.player.pos.x += self.player.horizontal_speed;
        }

        if self.up_pressed {
            self.player.pos.y -= self.player.vertical_speed;
        } else if self.down_pressed {
            self.player.pos.y += self.player.vertical_speed;
        }
    }


}

impl Scene for HelloScene {
    fn update(&mut self, ctx: &mut Context) {
        self.update_player_pos();

        self.player.update(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.player.draw(ctx);
    }

    fn on_key_down(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => self.up_pressed = true,
            Keycode::Down => self.down_pressed = true,
            Keycode::Left => self.left_pressed = true,
            Keycode::Right => self.right_pressed = true,
            Keycode::Escape => process::exit(0),
            _ => (),
        }
    }

    fn on_key_up(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => self.up_pressed = false,
            Keycode::Down => self.down_pressed = false,
            Keycode::Left => self.left_pressed = false,
            Keycode::Right => self.right_pressed = false,
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
                self.right_pressed = value > 7500;
                self.left_pressed = value < -7500;
            }
            Axis::LeftY => {
                self.down_pressed = value > 7500;
                self.up_pressed = value < -7500;
            }
            _ => (),
        }
    }
}
