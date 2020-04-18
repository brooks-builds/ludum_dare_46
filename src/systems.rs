use super::components::{
    Acceleration, HasGravity, Height, ObjectMesh, OnGround, Position, Velocity,
};
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context};
use specs::prelude::*;
use std::collections::HashSet;

pub struct GravitySystem {
    pub arena_height: f32,
}

impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        WriteStorage<'a, Acceleration>,
        ReadStorage<'a, HasGravity>,
        ReadStorage<'a, OnGround>,
    );

    fn run(&mut self, (mut acceleration, has_gravity, on_ground): Self::SystemData) {
        for (acceleration, _has_gravity, on_ground) in
            (&mut acceleration, &has_gravity, &on_ground).join()
        {
            if !on_ground.get() {
                acceleration.y += 0.1;
            }
        }
    }
}

pub struct ApplyForceSystem {
    pub delta_time: f32,
}

impl<'a> System<'a> for ApplyForceSystem {
    type SystemData = (
        WriteStorage<'a, Acceleration>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut acceleration, mut position, mut velocity): Self::SystemData) {
        let horizontal_limit = 10.0;
        let vertical_limit = 0.6;
        for (acceleration, position, velocity) in
            (&mut acceleration, &mut position, &mut velocity).join()
        {
            velocity.x += acceleration.x;
            velocity.y += acceleration.y;
            position.x += velocity.x * self.delta_time;
            position.y += velocity.y * self.delta_time;

            acceleration.x = 0.0;
            acceleration.y = 0.0;

            if velocity.x > horizontal_limit {
                println!("velocity x above limit");
                velocity.x = horizontal_limit;
            } else if velocity.x < -horizontal_limit {
                println!("velocity x below limit");
                velocity.x = -horizontal_limit;
            }

            // if velocity.y < -vertical_limit {
            //     println!("velocity y below limit");
            //     velocity.y = -vertical_limit;
            // }
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
    pub pressed_keys: &'a HashSet<KeyCode>,
}

impl<'a> System<'a> for MovePlayerSystem<'a> {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Acceleration>,
        ReadStorage<'a, OnGround>,
    );

    fn run(&mut self, (mut position, mut acceleration, on_ground): Self::SystemData) {
        let horizontal_speed = 1.5;
        for (position, acceleration, on_ground) in
            (&mut position, &mut acceleration, &on_ground).join()
        {
            if self.pressed_keys.contains(&KeyCode::A) || self.pressed_keys.contains(&KeyCode::Left)
            {
                acceleration.x -= horizontal_speed;
            } else if self.pressed_keys.contains(&KeyCode::D)
                || self.pressed_keys.contains(&KeyCode::Right)
            {
                acceleration.x += horizontal_speed;
            }

            if on_ground.get() && self.pressed_keys.contains(&KeyCode::Space) {
                acceleration.y -= 23.5;
            }
        }
    }
}

pub struct DragSystem;

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
            let mut force = Vector2::new(velocity.x, velocity.y);
            force = force.normalize();
            force *= -1.0;

            acceleration.x += force.x * 0.1;
        }
    }
}
