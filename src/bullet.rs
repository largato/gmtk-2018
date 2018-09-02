use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Point2, Rect};
use ggez::*;

use actor::*;

pub struct Bullet {
    pub color: Color,
    pub shape: Mesh,
    pub speed: f32,
    pub bounding_rect: Rect,
}

impl Actor for Bullet {
    fn update(&mut self, ctx: &mut Context) {
        self.bounding_rect.y += self.speed;
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::set_color(ctx, self.color).unwrap();
        graphics::draw(ctx, &self.shape, self.bounding_rect.point(), 0.0).unwrap();
    }
}

impl Bullet {
    pub fn build_bullet(ctx: &mut Context, color: Color, speed: f32, rect: Rect) -> Bullet {
        Bullet {
            color: color,
            shape: MeshBuilder::new()
                .ellipse(DrawMode::Fill, Point2::new(0.0, 0.0), rect.w, rect.h, 0.1)
                .build(ctx)
                .unwrap(),
            speed: speed,
            bounding_rect: rect,
        }
    }
}
