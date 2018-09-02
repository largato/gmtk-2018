extern crate ggez;

use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::graphics::{Color, Rect};
use ggez::*;

use std::process;

use actor::*;
use scene::*;
use ship::*;

pub struct HelloScene {
    player: Ship,
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
}

impl HelloScene {
    pub fn build_scene(ctx: &mut Context) -> HelloScene {
        let player_rect = Rect::new(100.0, 100.0, 20.0, 30.0);

        HelloScene {
            player: Ship::build_player(ctx, Color::from_rgb(255, 0, 0), 0.1, 0.08, player_rect),
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }

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
