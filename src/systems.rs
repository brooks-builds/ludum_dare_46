use super::components::{HasGravity, Height, ObjectMesh, Position};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context};
use specs::prelude::*;

pub struct GravitySystem {
    pub arena_height: f32,
}

impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, HasGravity>,
        ReadStorage<'a, Height>,
    );

    fn run(&mut self, (mut position, has_gravity, height): Self::SystemData) {
        use specs::Join;

        for (entity_position, _has_gravity, height) in (&mut position, &has_gravity, &height).join()
        {
            entity_position.y += 0.01;
            if entity_position.y + height.get() > self.arena_height {
                entity_position.y = self.arena_height - height.get();
            }
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
