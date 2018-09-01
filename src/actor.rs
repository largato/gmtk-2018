use ggez::*;

pub trait Actor {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, _ctx: &mut Context) {}
}
