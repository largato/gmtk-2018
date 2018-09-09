extern crate ggez;

use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::graphics::{Color, Rect};
use ggez::*;

use std::boxed::Box;
use std::process;
use std::vec::Vec;

use actor::*;
use bullet::*;
use scene::*;
use ship::*;

pub struct HelloScene {
    player: Ship,
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    actors: Vec<Box<Actor>>,
}

impl HelloScene {
    pub fn build_scene(ctx: &mut Context) -> HelloScene {
        let screen_width = ctx.conf.window_mode.width as f32;
        let screen_height = ctx.conf.window_mode.height as f32;
        let player_rect = Rect::new(screen_width / 2.0 - 10.0, screen_height - 60.0, 20.0, 30.0);

        let mut actors: Vec<Box<Actor>> = Vec::new();
        let h_enemies = 20;
        let v_enemies = 10;
        let h_space = screen_width / h_enemies as f32;
        let v_space = screen_height * 0.4 / v_enemies as f32;

        for x in 1..h_enemies - 1 {
            for y in 1..v_enemies - 1 {
                let enemy_rect = Rect::new(x as f32 * h_space, y as f32 * v_space, 15.0, 15.0);
                actors.push(Box::new(Ship::build_enemy(
                    ctx,
                    Color::from_rgb(0, 255, 0),
                    0.1,
                    0.08,
                    enemy_rect,
                )));
            }
        }

        HelloScene {
            player: Ship::build_player(ctx, Color::from_rgb(255, 0, 0), 3.5, 3.5, player_rect),
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
            actors: actors,
        }
    }

    fn update_player_pos(&mut self, ctx: &mut Context) {
        let mut new_x: f32 = self.player.pos_x();
        let mut new_y: f32 = self.player.pos_y();

        if self.left_pressed {
            new_x = self.player.pos_x() - self.player.horizontal_speed;
        } else if self.right_pressed {
            new_x = self.player.pos_x() + self.player.horizontal_speed;
        }

        if self.up_pressed {
            new_y = self.player.pos_y() - self.player.vertical_speed;
        } else if self.down_pressed {
            new_y = self.player.pos_y() + self.player.vertical_speed;
        }

        let screen_width = ctx.conf.window_mode.width as f32;
        let screen_height = ctx.conf.window_mode.height as f32;

        if new_x < 0.0 {
            new_x = 0.0;
        } else if new_x + self.player.bounding_rect.w > screen_width {
            new_x = screen_width - self.player.bounding_rect.w;
        }

        if new_y < 0.0 {
            new_y = 0.0;
        } else if new_y + self.player.bounding_rect.h > screen_height {
            new_y = screen_height - self.player.bounding_rect.h;
        }

        self.player.set_pos_x(new_x);
        self.player.set_pos_y(new_y);
    }

    fn shoot(&mut self, ctx: &mut Context) {
        let bullet_rect = Rect::new(
            self.player.pos_x() + self.player.bounding_rect.w / 2.0,
            self.player.pos_y() - 5.0,
            5.0,
            5.0,
        );
        self.actors.push(Box::new(Bullet::build_bullet(
            ctx,
            Color::from_rgb(255, 255, 255),
            -8.0,
            bullet_rect,
        )))
    }
}

impl Scene for HelloScene {
    fn update(&mut self, ctx: &mut Context) {
        // TODO: remove bullets on collisions and when they exit screen
        for actor in &mut self.actors {
            actor.update(ctx);
        }

        self.update_player_pos(ctx);

        self.player.update(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        for actor in &mut self.actors {
            actor.draw(ctx);
        }

        self.player.draw(ctx);
    }

    fn on_key_down(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Space => self.shoot(ctx),
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

    fn on_controller_button_down(&mut self, ctx: &mut Context, btn: Button, _ctrl_id: i32) {
        match btn {
            Button::A => self.shoot(ctx),
            Button::DPadUp => self.up_pressed = true,
            Button::DPadDown => self.down_pressed = true,
            Button::DPadLeft => self.left_pressed = true,
            Button::DPadRight => self.right_pressed = true,
            _ => (),
        }
    }

    fn on_controller_button_up(&mut self, _ctx: &mut Context, btn: Button, ctrl_id: i32) {
        match btn {
            Button::DPadUp => self.up_pressed = false,
            Button::DPadDown => self.down_pressed = false,
            Button::DPadLeft => self.left_pressed = false,
            Button::DPadRight => self.right_pressed = false,
            _ => (),
        }
    }

    fn on_controller_axis(&mut self, _ctx: &mut Context, axis: Axis, value: i16, _ctrl_id: i32) {
        match axis {
            Axis::LeftX => {
                self.right_pressed = value > 1000;
                self.left_pressed = value < -1000;
            }
            Axis::LeftY => {
                self.down_pressed = value > 1000;
                self.up_pressed = value < -1000;
            }
            _ => (),
        }
    }
}
