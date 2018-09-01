extern crate ggez;

use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::*;

mod scene;
mod helloscene;

use scene::*;
use helloscene::*;

struct MainState {
    scene_manager: SceneManager
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            scene_manager: SceneManager {
                current: Box::new(HelloScene { pos_x: 0.0, pos_y: 0.0 })
            },
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.scene_manager.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.scene_manager.draw(ctx);

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.scene_manager.current.on_key_down(ctx, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.scene_manager.current.on_key_up(ctx, keycode, keymod, repeat);
    }

    fn controller_button_down_event(&mut self, ctx: &mut Context, btn: Button, ctrl_id: i32) {
        self.scene_manager.current.on_controller_button_down(ctx, btn, ctrl_id);
    }

    fn controller_button_up_event(&mut self, ctx: &mut Context, btn: Button, ctrl_id: i32) {
        self.scene_manager.current.on_controller_button_up(ctx, btn, ctrl_id);
    }

    fn controller_axis_event(&mut self, ctx: &mut Context, axis: Axis, value: i16, ctrl_id: i32) {
        self.scene_manager.current.on_controller_axis(ctx, axis, value, ctrl_id);
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
