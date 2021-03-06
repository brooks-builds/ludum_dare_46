mod components;
mod meshes;
mod resources;
mod systems;

use components::{
    Acceleration, Bullet, BulletState, Drag, Floor, Flyer, HasGravity, Height, KeepAlive,
    ObjectMesh, OnGround, Player, Position, Radius, Velocity, Width,
};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::input::{keyboard, mouse};
use ggez::nalgebra::Point2;
use ggez::{graphics, timer, Context, GameResult};
use rand::prelude::*;
use resources::{BulletSize, DelayFiringUntilAfter, Score, StillAlive};
use specs::prelude::*;
use specs::ReadStorage;
use systems::{
    ApplyForceSystem, CheckEggSystem, DragSystem, FireBulletSystem, FlySystem, GravitySystem,
    HideHitBullets, HitGround, IncreaseScoreBySurvivingSystem, LandOnEggSystem, MovePlayerSystem,
    RenderSystem, ResetBulletsSystem, ShootBirdsSystem,
};

pub struct GameState {
    world: World,
    create_bird_every_miliseconds: u128,
    time_to_create_bird: u128,
    birds_to_create_at_the_same_time: usize,
    bird_mesh: Mesh,
    bird_width: f32,
    bird_height: f32,
    increase_score_every_miliseconds: u128,
    next_score_increase_time: u128,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<GameState> {
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let egg_width = 5.0;
        let egg_height = 15.0;
        let egg_mesh = meshes::createEggMesh(context, egg_width, egg_height)?;
        let player_height = 50.0;
        let player_width = 15.0;
        let player = meshes::createPersonMesh(context, player_width, player_height)?;
        let floor = meshes::createFloor(context, arena_width, 5.0)?;
        let mut world = World::new();
        let bullet_size = 5.0;
        let bird_width = 25.0;
        let bird_height = 10.0;
        world.register::<Position>();
        world.register::<ObjectMesh>();
        world.register::<HasGravity>();
        world.register::<Floor>();
        world.register::<Height>();
        world.register::<Velocity>();
        world.register::<Acceleration>();
        world.register::<Drag>();
        world.register::<OnGround>();
        world.register::<Width>();
        world.register::<KeepAlive>();
        world.register::<Flyer>();
        world.register::<Player>();
        world.register::<Bullet>();
        world.register::<Radius>();
        world.register::<BulletState>();

        world.insert(StillAlive::new());
        world.insert(BulletSize::new(bullet_size));
        world.insert(DelayFiringUntilAfter::new());
        world.insert(Score::new());

        // egg
        world
            .create_entity()
            .with(Position {
                x: arena_width / 2.0,
                y: arena_height - 25.0,
            })
            .with(ObjectMesh::new(egg_mesh))
            .with(Width::new(egg_width))
            .with(Height::new(egg_height))
            .with(KeepAlive::new())
            .build();

        // player
        world
            .create_entity()
            .with(Position {
                x: 100.0,
                y: arena_height - player_width - 500.0,
            })
            .with(ObjectMesh::new(player))
            .with(HasGravity)
            .with(Height::new(player_height / 2.0))
            .with(Width::new(player_width))
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(Acceleration { x: 0.0, y: 0.0 })
            .with(Drag::new(0.0))
            .with(OnGround::new())
            .with(Player)
            .build();

        // floor
        world
            .create_entity()
            .with(Position {
                x: 0.0,
                y: arena_height - 5.0,
            })
            .with(ObjectMesh::new(floor))
            .with(Floor)
            .build();

        // bullets
        for _ in 0..3 {
            let bullet_mesh = meshes::createBullet(context, bullet_size)?;
            world
                .create_entity()
                .with(Position { x: -50.0, y: -50.0 })
                .with(ObjectMesh::new(bullet_mesh))
                .with(Velocity { x: 0.0, y: 0.0 })
                .with(Bullet)
                .with(Radius::new(bullet_size))
                .with(BulletState::new())
                .build();
        }
        Ok(GameState {
            world,
            create_bird_every_miliseconds: 3000,
            time_to_create_bird: 0,
            birds_to_create_at_the_same_time: 1,
            bird_mesh: meshes::createBird(context, bird_width, bird_height)?,
            bird_width,
            bird_height,
            increase_score_every_miliseconds: 5000,
            next_score_increase_time: 5000,
        })
    }
}

impl GameState {
    fn create_bird(&mut self, arena_width: f32, arena_height: f32) -> GameResult<()> {
        for _ in 0..self.birds_to_create_at_the_same_time {
            let mut rng = rand::thread_rng();
            self.world
                .create_entity()
                .with(Position {
                    x: rng.gen_range(-self.bird_width, arena_width + self.bird_width),
                    y: -self.bird_height - 10.0,
                })
                .with(ObjectMesh::new(self.bird_mesh.clone()))
                .with(Height::new(self.bird_height))
                .with(Width::new(self.bird_width))
                .with(Velocity { x: 0.0, y: 0.0 })
                .with(Acceleration { x: 0.0, y: 0.0 })
                .with(Drag::new(0.0))
                .with(Flyer)
                .build();
        }
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let mut delta_time = ggez::timer::average_delta(context).as_secs_f32();
        let pressed_keys = keyboard::pressed_keys(context);
        let fps_cap = 1.0 / 60.0;
        if delta_time < fps_cap {
            delta_time = fps_cap;
        }
        let duration_since_start = timer::time_since_start(context).as_millis();
        let mut gravity_system = GravitySystem { arena_height };
        let mut move_system = ApplyForceSystem { delta_time };
        let mut hit_ground = HitGround { arena_height };
        let mut move_player_system = MovePlayerSystem { pressed_keys };
        let mut drag_system = DragSystem;
        let mut check_egg = CheckEggSystem;
        let mut fly_system = FlySystem;
        let mut landing_on_egg = LandOnEggSystem;
        let mut reset_bullets = ResetBulletsSystem {
            arena_width,
            arena_height,
        };
        let mut shoot_bird_system = ShootBirdsSystem;
        let mut hide_hit_bullets = HideHitBullets;
        if mouse::button_pressed(context, mouse::MouseButton::Left) {
            let mouse_location = mouse::position(context);
            let mut fire_bullet_system = FireBulletSystem {
                mouse_location: Point2::new(mouse_location.x, mouse_location.y),
                duration_since_start: duration_since_start,
            };
            fire_bullet_system.run_now(&mut self.world);
        }

        if self.time_to_create_bird < duration_since_start {
            self.create_bird(arena_width, arena_height)?;
            self.time_to_create_bird = duration_since_start + self.create_bird_every_miliseconds;
            if self.birds_to_create_at_the_same_time < 50 {
                self.birds_to_create_at_the_same_time += 1;
            }
        }

        if self.next_score_increase_time < duration_since_start {
            let mut score_system = IncreaseScoreBySurvivingSystem;
            score_system.run_now(&self.world);
            self.next_score_increase_time =
                duration_since_start + self.increase_score_every_miliseconds;
        }

        gravity_system.run_now(&self.world);
        hit_ground.run_now(&self.world);
        move_system.run_now(&self.world);
        move_player_system.run_now(&self.world);
        drag_system.run_now(&self.world);
        check_egg.run_now(&self.world);
        fly_system.run_now(&self.world);
        landing_on_egg.run_now(&self.world);
        reset_bullets.run_now(&self.world);
        shoot_bird_system.run_now(&self.world);
        hide_hit_bullets.run_now(&self.world);

        // let still_alive = &self.world.fetch::<StillAlive>();

        // if !still_alive.get() {
        //     println!("game over");
        // }
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let mut draw_system = RenderSystem { context };
        draw_system.run_now(&self.world);

        graphics::present(context)
    }
}
