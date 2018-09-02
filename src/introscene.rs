extern crate ggez;

use ggez::event::{Button, Keycode, Mod};
use ggez::*;

use scene::*;

pub struct IntroScene {
    duration: u32,
    image: graphics::Image,
}

impl Scene for IntroScene {
    fn update(&mut self, ctx: &mut Context) {
        let time_elapsed = timer::get_delta(ctx).subsec_millis() as u32;

        if time_elapsed < self.duration {
            self.duration -= time_elapsed;
        } else {
            // End scene
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        let screen_width = ctx.conf.window_mode.width;
        let screen_height = ctx.conf.window_mode.height;

        graphics::set_background_color(ctx, (255, 255, 255, 255).into());
        graphics::draw(
            ctx,
            &self.image,
            graphics::Point2::new(
                ((screen_width - self.image.width()) as f32) / 2.0,
                ((screen_height - self.image.height()) as f32) / 2.0,
            ),
            0.0,
        ).unwrap();
    }

    fn on_key_down(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        // End scene
    }

    fn on_controller_button_down(&mut self, _ctx: &mut Context, _btn: Button, _ctrl_id: i32) {
        // End scene
    }
}

impl IntroScene {
    pub fn build_scene(ctx: &mut Context) -> IntroScene {
        let img = graphics::Image::new(ctx, "/sprites/largato_logo.png").unwrap();
        IntroScene {
            duration: 10_000,
            image: img,
        }
    }
}
