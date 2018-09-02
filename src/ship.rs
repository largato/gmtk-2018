use ggez::graphics::{Rect, Color, Mesh, MeshBuilder, Point2};
use ggez::*;

use actor::*;

pub struct Ship {
    pub pos: Point2,
    pub color: Color,
    pub shape: Mesh,
    pub horizontal_speed: f32,
    pub vertical_speed: f32,
}

impl Actor for Ship {
    fn draw(&mut self, ctx: &mut Context) {
        graphics::set_color(ctx, self.color).unwrap();
        graphics::draw(ctx, &self.shape, self.pos, 0.0).unwrap();
    }
}

impl Ship {
    pub fn build_player(ctx: &mut Context, color: Color, horizontal_speed: f32, vertical_speed: f32, rect: Rect) -> Ship {
        Ship {
            pos: Point2::new(rect.x, rect.y),
            color: color,
            horizontal_speed: horizontal_speed,
            vertical_speed: vertical_speed,
            shape:  MeshBuilder::new()
                .line(&[
                      Point2::new(rect.left(), rect.bottom()),
                      Point2::new(rect.right(), rect.bottom()),
                      Point2::new(rect.left() + rect.w / 2.0, rect.top()),
                      Point2::new(rect.left(), rect.bottom()),
                ], 4.0).build(ctx).unwrap(),
        }
    }
}
