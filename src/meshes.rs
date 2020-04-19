use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub fn createEggMesh(context: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
    MeshBuilder::new()
        .ellipse(
            DrawMode::fill(),
            Point2::new(0.0, 0.0),
            width,
            height,
            1.5,
            graphics::WHITE,
        )
        .build(context)
}

pub fn createPersonMesh(context: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
    let body = Rect::new(0.0 - (width / 2.0), 0.0 - (height / 2.0), width, height);
    MeshBuilder::new()
        .rectangle(DrawMode::fill(), body, graphics::WHITE)
        .circle(
            DrawMode::fill(),
            Point2::new(0.0, 0.0 - height / 2.0),
            15.0,
            0.1,
            graphics::WHITE,
        )
        .build(context)
}

pub fn createFloor(context: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
    let floor = Rect::new(0.0, 0.0, width, height);
    MeshBuilder::new()
        .rectangle(DrawMode::fill(), floor, graphics::WHITE)
        .build(context)
}

pub fn createBird(context: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
    MeshBuilder::new()
        .polyline(
            DrawMode::stroke(5.0),
            &[
                Point2::new(-width, -height),
                Point2::new(0.0, 0.0),
                Point2::new(width, -height),
            ],
            graphics::WHITE,
        )?
        .build(context)
}

pub fn createBullet(context: &mut Context, radius: f32) -> GameResult<Mesh> {
    MeshBuilder::new()
        .circle(
            DrawMode::fill(),
            Point2::new(0.0, 0.0),
            radius,
            0.1,
            graphics::WHITE,
        )
        .build(context)
}
