extern crate ggez;

use ggez::*;

use scene::*;

pub struct IntroScene {
    duration: u32,
    image: graphics::Image,
}

impl Scene for IntroScene {
    fn update(&mut self, _ctx: &mut Context) {
        // Countdown
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
    pub fn build_intro(ctx: &mut Context, duration: u32) -> IntroScene {
        let img = graphics::Image::new(ctx, "/sprites/largato_logo.png").unwrap();
        IntroScene {
            duration: duration,
            image: img,
        }
    }
}
