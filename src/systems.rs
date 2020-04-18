use super::components::{Acceleration, HasGravity, Height, ObjectMesh, Position, Velocity};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context};
use specs::prelude::*;

pub struct GravitySystem {
    pub arena_height: f32,
    pub delta_time: f32,
}

impl<'a> System<'a> for GravitySystem {
    type SystemData = (WriteStorage<'a, Acceleration>, ReadStorage<'a, HasGravity>);

    fn run(&mut self, (mut acceleration, has_gravity): Self::SystemData) {
        use specs::Join;

        for (acceleration, _has_gravity) in (&mut acceleration, &has_gravity).join() {
            acceleration.y += 1.0 * self.delta_time;
        }
    }
}

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        WriteStorage<'a, Acceleration>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut acceleration, mut position, mut velocity): Self::SystemData) {
        for (acceleration, position, velocity) in
            (&mut acceleration, &mut position, &mut velocity).join()
        {
            velocity.x += acceleration.x;
            velocity.y += acceleration.y;
            position.x += velocity.x;
            position.y += velocity.y;

            acceleration.x = 0.0;
            acceleration.y = 0.0;
        }
    }
}

pub struct RenderSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, ObjectMesh>);

    fn run(&mut self, (position, mesh): Self::SystemData) {
        use specs::Join;

        for (position, mesh) in (&position, &mesh).join() {
            graphics::draw(
                self.context,
                mesh.get(),
                graphics::DrawParam::default().dest(Point2::new(position.x, position.y)),
            )
            .unwrap();
        }
    }
}
