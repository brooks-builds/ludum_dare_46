use super::components::{
    Acceleration, HasGravity, Height, ObjectMesh, OnGround, Position, Velocity,
};
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
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

pub struct ApplyForceSystem;

impl<'a> System<'a> for ApplyForceSystem {
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

            if velocity.x > 1.0 {
                velocity.x = 1.0;
            }

            if velocity.y > 1.0 {
                velocity.y = 1.0;
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

pub struct HitGround {
    pub arena_height: f32,
}

impl<'a> System<'a> for HitGround {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Height>,
        ReadStorage<'a, HasGravity>,
        WriteStorage<'a, OnGround>,
    );

    fn run(&mut self, (mut position, height, has_gravity, mut on_ground): Self::SystemData) {
        for (position, height, _has_gravity, on_ground) in
            (&mut position, &height, &has_gravity, &mut on_ground).join()
        {
            if position.y + height.get() > self.arena_height {
                position.y = self.arena_height - height.get();
                on_ground.set(true);
            } else {
                on_ground.set(false);
            }
        }
    }
}

pub struct MovePlayerSystem<'a> {
    pub context: &'a mut Context,
    pub delta_time: f32,
}

impl<'a> System<'a> for MovePlayerSystem<'a> {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Acceleration>);

    fn run(&mut self, (mut position, mut acceleration): Self::SystemData) {
        for (position, acceleration) in (&mut position, &mut acceleration).join() {
            if keyboard::is_key_pressed(self.context, KeyCode::A)
                || keyboard::is_key_pressed(self.context, KeyCode::Left)
            {
                acceleration.x -= 8.0 * self.delta_time;
            } else if keyboard::is_key_pressed(self.context, KeyCode::D)
                || keyboard::is_key_pressed(self.context, KeyCode::Right)
            {
                acceleration.x += 8.0 * self.delta_time;
            }
        }
    }
}

pub struct DragSystem {
    pub delta_time: f32,
}

impl<'a> System<'a> for DragSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Acceleration>,
        ReadStorage<'a, OnGround>,
    );

    fn run(&mut self, (mut velocity, mut acceleration, on_ground): Self::SystemData) {
        for (velocity, acceleration, on_ground) in
            (&mut velocity, &mut acceleration, &on_ground).join()
        {
            if on_ground.get() {
                let mut force = Vector2::new(velocity.x, velocity.y);
                force *= -1.0;
                force = force.normalize();

                acceleration.x += force.x * 0.01;
            }
        }
    }
}
