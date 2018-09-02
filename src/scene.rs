extern crate ggez;

use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::*;

use std::boxed::Box;

pub struct SceneManager {
    stack: Vec<Box<Scene>>,
}

impl SceneManager {
    pub fn build_manager() -> SceneManager {
        SceneManager { stack: Vec::new() }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        self.current().unwrap().update(_ctx);
    }

    pub fn draw(&mut self, _ctx: &mut Context) {
        self.current().unwrap().draw(_ctx);
    }

    pub fn on_key_down(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.current()
            .unwrap()
            .on_key_down(ctx, keycode, keymod, repeat);
    }

    pub fn on_key_up(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.current()
            .unwrap()
            .on_key_up(ctx, keycode, keymod, repeat);
    }

    pub fn on_controller_button_down(&mut self, ctx: &mut Context, btn: Button, ctrl_id: i32) {
        self.current()
            .unwrap()
            .on_controller_button_down(ctx, btn, ctrl_id);
    }

    pub fn on_controller_button_up(&mut self, ctx: &mut Context, btn: Button, ctrl_id: i32) {
        self.current()
            .unwrap()
            .on_controller_button_up(ctx, btn, ctrl_id);
    }

    pub fn on_controller_axis(&mut self, ctx: &mut Context, axis: Axis, value: i16, ctrl_id: i32) {
        self.current()
            .unwrap()
            .on_controller_axis(ctx, axis, value, ctrl_id);
    }

    pub fn push(&mut self, scene: Box<Scene>) {
        self.stack.push(scene);
    }

    pub fn pop(&mut self) -> Option<Box<Scene>> {
        self.stack.pop()
    }

    fn current(&mut self) -> Result<&mut Box<Scene>, String> {
        match self.stack.len() {
            0 => Err(String::from("Stack vazia, caraio")),
            n => Ok(&mut self.stack[n - 1]),
        }
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
