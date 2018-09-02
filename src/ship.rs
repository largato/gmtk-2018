use ggez::graphics::{Color, Mesh, MeshBuilder, Point2, Rect};
use ggez::*;

use actor::*;

pub struct Ship {
    pub color: Color,
    pub shape: Mesh,
    pub horizontal_speed: f32,
    pub vertical_speed: f32,
    pub bounding_rect: Rect,
}

impl Actor for Ship {
    fn draw(&mut self, ctx: &mut Context) {
        graphics::set_color(ctx, self.color).unwrap();
        graphics::draw(ctx, &self.shape, self.bounding_rect.point(), 0.0).unwrap();
    }
}

impl Ship {
    pub fn build_player(
        ctx: &mut Context,
        color: Color,
        horizontal_speed: f32,
        vertical_speed: f32,
        rect: Rect,
    ) -> Ship {
        Ship {
            color: color,
            horizontal_speed: horizontal_speed,
            vertical_speed: vertical_speed,
            shape: MeshBuilder::new()
                .line(
                    &[
                        Point2::new(0.0, rect.h),
                        Point2::new(rect.w, rect.h),
                        Point2::new(rect.w / 2.0, 0.0),
                        Point2::new(0.0, rect.h),
                    ],
                    4.0,
                )
                .build(ctx)
                .unwrap(),
            bounding_rect: rect,
        }
    }

    pub fn pos_x(&mut self) -> f32 {
        self.bounding_rect.x as f32
    }

    pub fn pos_y(&mut self) -> f32 {
        self.bounding_rect.y as f32
    }

    pub fn set_pos_x(&mut self, x: f32) {
        self.bounding_rect.x = x;
    }

    pub fn set_pos_y(&mut self, y: f32) {
        self.bounding_rect.y = y;
    }
}
