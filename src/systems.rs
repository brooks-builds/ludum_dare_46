use super::components::{
    Acceleration, Bullet, BulletState, CurrentBulletState, Flyer, HasGravity, Height, KeepAlive,
    ObjectMesh, OnGround, Player, Position, Radius, Velocity, Width,
};
use super::meshes;
use super::resources::{BulletSize, DelayFiringUntilAfter, Score, StillAlive};
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context};
use specs::prelude::*;
use specs::Entities;
use std::collections::HashSet;
use std::time::Duration;

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
                acceleration.y += 0.5;
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
        Read<'a, StillAlive>,
    );

    fn run(
        &mut self,
        (mut acceleration, mut position, mut velocity, still_alive): Self::SystemData,
    ) {
        if still_alive.get() {
            let horizontal_limit = 10.0;
            let vertical_limit = 0.6;
            for (acceleration, position, velocity) in
                ((&mut acceleration).maybe(), &mut position, &mut velocity).join()
            {
                if let Some(acceleration) = acceleration {
                    velocity.x += acceleration.x;
                    velocity.y += acceleration.y;
                    acceleration.x = 0.0;
                    acceleration.y = 0.0;
                }
                position.x += velocity.x * self.delta_time;
                position.y += velocity.y * self.delta_time;
            }
        }
    }
}

pub struct RenderSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderSystem<'_> {
    fn draw_score_small(&mut self, score: usize) {
        let text = graphics::Text::new(format!("Score: {}", score));

        graphics::draw(
            self.context,
            &text,
            graphics::DrawParam::default().dest(Point2::new(5.0, 25.0)),
        )
        .unwrap();
    }

    fn draw_score_large(&mut self, score: usize, arena_width: f32, arena_height: f32) {
        let mut text = graphics::Text::new(format!("You Scored {}", score));
        let font = graphics::Font::default();
        let font_scale = graphics::Scale::uniform(100.0);

        text.set_font(font, font_scale);

        graphics::draw(
            self.context,
            &text,
            graphics::DrawParam::default()
                .dest(Point2::new(arena_width / 5.0, arena_height / 2.0 + 50.0)),
        )
        .unwrap();
    }
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, ObjectMesh>,
        Read<'a, StillAlive>,
        ReadStorage<'a, BulletState>,
        Read<'a, Score>,
    );

    fn run(&mut self, (position, mesh, still_alive, bullet_state, score): Self::SystemData) {
        let mut bullet_count_text = String::from("Bullets: ");
        for bullet_state in bullet_state.join() {
            if let CurrentBulletState::Ready = bullet_state.get() {
                bullet_count_text = format!("{} *", bullet_count_text);
            }
        }

        for (position, mesh) in (&position, &mesh).join() {
            graphics::draw(
                self.context,
                mesh.get(),
                graphics::DrawParam::default().dest(Point2::new(position.x, position.y)),
            )
            .unwrap();
        }
        if !still_alive.get() {
            let (arena_width, arena_height) = graphics::drawable_size(self.context);
            let font = graphics::Font::default();
            let font_scale = graphics::Scale::uniform(100.0);
            let mut game_over_text = graphics::Text::new("Game Over");
            game_over_text.set_font(font, font_scale);
            graphics::draw(
                self.context,
                &game_over_text,
                graphics::DrawParam::default()
                    .dest(Point2::new(arena_width / 4.0, arena_height / 2.0 - 100.0)),
            )
            .unwrap();
            self.draw_score_large(score.get(), arena_width, arena_height);
        } else {
            self.draw_score_small(score.get());
        }

        graphics::draw(
            self.context,
            &graphics::Text::new(bullet_count_text),
            graphics::DrawParam::default().dest(Point2::new(5.0, 5.0)),
        )
        .unwrap();
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
        let horizontal_speed = 0.8;
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
                acceleration.y -= 50.5;
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

            acceleration.x += force.x * 0.8;
        }
    }
}

pub struct CheckEggSystem;

impl<'a> System<'a> for CheckEggSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Height>,
        ReadStorage<'a, Width>,
        WriteStorage<'a, KeepAlive>,
        Write<'a, StillAlive>,
    );

    fn run(
        &mut self,
        (position, height, width, mut keep_alive, mut still_alive): Self::SystemData,
    ) {
        for (entity_position, entity_height, entity_width, ()) in
            (&position, &height, &width, !&keep_alive).join()
        {
            for (egg_position, egg_height, egg_width, keep_alive) in
                (&position, &height, &width, &keep_alive).join()
            {
                // should only be the egg
                let entity_location = Vector2::new(entity_position.x, entity_position.y);
                let egg_location = Vector2::new(egg_position.x, egg_position.y);
                let distance = entity_location - egg_location;
                let distance = distance.magnitude();

                if distance < egg_width.get() {
                    still_alive.set(false);
                }
            }
        }
    }
}

pub struct FlySystem;

impl<'a> System<'a> for FlySystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, KeepAlive>,
        WriteStorage<'a, Acceleration>,
        ReadStorage<'a, Flyer>,
    );

    fn run(&mut self, (position, keep_alive, mut acceleration, flyer): Self::SystemData) {
        for (flyer_position, flyer_acceleration, _flyer) in
            (&position, &mut acceleration, &flyer).join()
        {
            let flyer_location = Vector2::new(flyer_position.x, flyer_position.y);

            for (egg_position, _keep_alive) in (&position, &keep_alive).join() {
                let egg_location = Vector2::new(egg_position.x, egg_position.y);
                let mut direction = egg_location - flyer_location;
                let distance = direction.magnitude();

                if distance > 25.0 {
                    direction = direction.normalize();
                    let force = direction * 0.05;
                    flyer_acceleration.x += force.x;
                    flyer_acceleration.y += force.y;
                }
            }
        }
    }
}

pub struct LandOnEggSystem;

impl<'a> System<'a> for LandOnEggSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Flyer>,
        ReadStorage<'a, KeepAlive>,
        WriteStorage<'a, Acceleration>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (position, flyer, keep_alive, mut acceleration, mut velocity): Self::SystemData,
    ) {
        for (flyer_position, flyer, flyer_acceleration, flyer_velocity) in
            (&position, &flyer, &mut acceleration, &mut velocity).join()
        {
            let flyer_location = Vector2::new(flyer_position.x, flyer_position.y);

            for (egg_position, keep_alive) in (&position, &keep_alive).join() {
                let egg_location = Vector2::new(egg_position.x, egg_position.y);
                let direction = egg_location - flyer_location;
                let distance = direction.magnitude();

                if distance < 2.0 {
                    flyer_acceleration.x = 0.0;
                    flyer_acceleration.y = 0.0;
                    flyer_velocity.x = 0.0;
                    flyer_velocity.y = 0.0;
                }
            }
        }
    }
}

pub struct FireBulletSystem {
    pub mouse_location: Point2<f32>,
    pub duration_since_start: u128,
}

impl<'a> System<'a> for FireBulletSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        Read<'a, BulletSize>,
        ReadStorage<'a, Bullet>,
        WriteStorage<'a, BulletState>,
        Write<'a, DelayFiringUntilAfter>,
    );

    fn run(
        &mut self,
        (
            mut position,
            player,
            mut velocity,
            bullet_size,
            bullet,
            mut bullet_state,
            mut delay_firing_until_after,
        ): Self::SystemData,
    ) {
        let mut player_location = Vector2::new(-50.0, -50.0);
        let mut direction = Vector2::new(0.0, 0.0);
        let bullet_speed = 150.0;
        for (player_position, _player) in (&position, &player).join() {
            player_location = Vector2::new(player_position.x, player_position.y);
            let target_location = Vector2::new(self.mouse_location.x, self.mouse_location.y);
            direction = target_location - player_location;
        }
        for (bullet_position, _bullet, bullet_velocity, bullet_state) in
            (&mut position, &bullet, &mut velocity, &mut bullet_state).join()
        {
            if let CurrentBulletState::Ready = bullet_state.get() {
                if delay_firing_until_after.get() < self.duration_since_start {
                    direction = direction.normalize();
                    bullet_position.x = player_location.x;
                    bullet_position.y = player_location.y;
                    bullet_velocity.x = direction.x * bullet_speed;
                    bullet_velocity.y = direction.y * bullet_speed;
                    bullet_state.fire();
                    delay_firing_until_after.set(self.duration_since_start + 100);
                }
            }
        }
    }
}

pub struct ResetBulletsSystem {
    pub arena_width: f32,
    pub arena_height: f32,
}

impl<'a> System<'a> for ResetBulletsSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Bullet>,
        WriteStorage<'a, BulletState>,
    );

    fn run(&mut self, (position, mut velocity, bullet, mut bullet_state): Self::SystemData) {
        for (position, velocity, bullet, bullet_state) in
            (&position, &mut velocity, &bullet, &mut bullet_state).join()
        {
            if position.x < -10.0
                || position.x > self.arena_width + 10.0
                || position.y < -10.0
                || position.y > self.arena_height + 10.0
            {
                bullet_state.ready();
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
        }
    }
}

pub struct ShootBirdsSystem;

impl<'a> System<'a> for ShootBirdsSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Bullet>,
        ReadStorage<'a, Flyer>,
        Entities<'a>,
        WriteStorage<'a, BulletState>,
        Write<'a, Score>,
    );

    fn run(
        &mut self,
        (mut position, bullet, flyer, entities, mut bullet_state, mut score): Self::SystemData,
    ) {
        for (bullet_position, bullet, bullet_entity, bullet_state) in
            (&position, &bullet, &entities, &mut bullet_state).join()
        {
            if let CurrentBulletState::Firing = bullet_state.get() {
                let bullet_location = Vector2::new(bullet_position.x, bullet_position.y);
                for (flyer_position, flyer, flyer_entity) in (&position, &flyer, &entities).join() {
                    let flyer_location = Vector2::new(flyer_position.x, flyer_position.y);
                    let direction = flyer_location - bullet_location;
                    let distance = direction.magnitude();
                    if distance < 25.0 {
                        entities.delete(flyer_entity).unwrap();
                        bullet_state.hit();
                        score.increase(10);
                    }
                }
            }
        }
    }
}

pub struct HideHitBullets;

impl<'a> System<'a> for HideHitBullets {
    type SystemData = (
        ReadStorage<'a, Bullet>,
        WriteStorage<'a, BulletState>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (bullet, mut bullet_state, mut position): Self::SystemData) {
        for (bullet, bullet_state, position) in (&bullet, &mut bullet_state, &mut position).join() {
            if let CurrentBulletState::Hit = bullet_state.get() {
                bullet_state.ready();
                position.x = -50.0;
                position.y = -50.0;
            }
        }
    }
}

pub struct IncreaseScoreBySurvivingSystem;

impl<'a> System<'a> for IncreaseScoreBySurvivingSystem {
    type SystemData = (Read<'a, StillAlive>, Write<'a, Score>);

    fn run(&mut self, (still_alive, mut score): Self::SystemData) {
        if still_alive.get() {
            score.increase(1);
        }
    }
}
