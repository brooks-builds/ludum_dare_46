use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub fn createEggMesh(context: &mut Context) -> GameResult<Mesh> {
    MeshBuilder::new()
        .ellipse(
            DrawMode::fill(),
            Point2::new(0.0, 0.0),
            10.0,
            25.0,
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
