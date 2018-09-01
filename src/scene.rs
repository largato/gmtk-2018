extern crate ggez;

use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::*;

use std::boxed::Box;

pub struct SceneManager {
    pub current: Box<Scene>,

    // TODO: scene stack?
}

impl SceneManager {
    pub fn update(&mut self, _ctx: &mut Context) {
        self.current.update(_ctx);
    }

    pub fn draw(&mut self, _ctx: &mut Context) {
        self.current.draw(_ctx);
    }
}

pub trait Scene {
    fn update(&mut self, ctx: &mut Context);

    fn draw(&mut self, ctx: &mut Context);

    fn on_key_up(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {}
    fn on_key_down(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {}
    fn on_controller_button_down(&mut self, _ctx: &mut Context, _btn: Button, _ctrl_id: i32) {}
    fn on_controller_button_up(&mut self, _ctx: &mut Context, _btn: Button, _ctrl_id: i32) {}
    fn on_controller_axis(&mut self, _ctx: &mut Context, _axis: Axis, _value: i16, _ctrl_id: i32) {}
}
