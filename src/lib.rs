mod components;
mod meshes;
mod resources;
mod systems;

use components::{
    Acceleration, Drag, Floor, Flyer, HasGravity, Height, KeepAlive, ObjectMesh, OnGround,
    Position, Velocity, Width,
};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use resources::StillAlive;
use specs::prelude::*;
use specs::ReadStorage;
use systems::{
    ApplyForceSystem, CheckEggSystem, DragSystem, FlySystem, GravitySystem, HitGround,
    LandOnEggSystem, MovePlayerSystem, RenderSystem,
};

pub struct GameState {
    world: World,
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
        let bird_width = 25.0;
        let bird_height = 10.0;
        let bird_mesh = meshes::createBird(context, bird_width, bird_height)?;
        let mut world = World::new();
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

        world.insert(StillAlive::new());

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

        // bird
        world
            .create_entity()
            .with(Position { x: 200.0, y: 200.0 })
            .with(ObjectMesh::new(bird_mesh))
            .with(Height::new(bird_height))
            .with(Width::new(bird_width))
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(Acceleration { x: 0.0, y: 0.0 })
            .with(Drag::new(0.0))
            .with(Flyer)
            .build();
        Ok(GameState { world })
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
        let mut gravity_system = GravitySystem { arena_height };
        let mut move_system = ApplyForceSystem { delta_time };
        let mut hit_ground = HitGround { arena_height };
        let mut move_player_system = MovePlayerSystem { pressed_keys };
        let mut drag_system = DragSystem;
        let mut check_egg = CheckEggSystem;
        let mut fly_system = FlySystem;
        let mut landing_on_egg = LandOnEggSystem;

        gravity_system.run_now(&self.world);
        hit_ground.run_now(&self.world);
        move_system.run_now(&self.world);
        move_player_system.run_now(&self.world);
        drag_system.run_now(&self.world);
        check_egg.run_now(&self.world);
        fly_system.run_now(&self.world);
        landing_on_egg.run_now(&self.world);

        let still_alive = self.world.fetch::<StillAlive>();

        if !still_alive.get() {
            println!("game over");
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let mut draw_system = RenderSystem { context };
        draw_system.run_now(&self.world);

        graphics::present(context)
    }
}
