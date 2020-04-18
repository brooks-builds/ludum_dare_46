mod components;
mod meshes;
mod systems;

use components::{Acceleration, Drag, Floor, HasGravity, Height, ObjectMesh, Position, Velocity};
use ggez::event::EventHandler;
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use specs::prelude::*;
use systems::{GravitySystem, MoveSystem, RenderSystem};

pub struct GameState {
    world: World,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<GameState> {
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let egg_mesh = meshes::createEggMesh(context)?;
        let player_height = 50.0;
        let player_width = 15.0;
        let player = meshes::createPersonMesh(context, player_width, player_height)?;
        let floor = meshes::createFloor(context, arena_width, 5.0)?;
        let mut world = World::new();
        world.register::<Position>();
        world.register::<ObjectMesh>();
        world.register::<HasGravity>();
        world.register::<Floor>();
        world.register::<Height>();
        world.register::<Velocity>();
        world.register::<Acceleration>();
        world.register::<Drag>();

        // egg
        world
            .create_entity()
            .with(Position {
                x: arena_width / 2.0,
                y: arena_height - 25.0,
            })
            .with(ObjectMesh::new(egg_mesh))
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
            .with(Velocity { x: 0.0, y: 0.0 })
            .with(Acceleration { x: 0.0, y: 0.0 })
            .with(Drag::new(0.0))
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
        Ok(GameState { world })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let delta_time = ggez::timer::delta(context);
        let mut gravity_system = GravitySystem {
            arena_height,
            delta_time: delta_time.as_secs_f32(),
        };
        let mut move_system = MoveSystem;
        gravity_system.run_now(&self.world);
        move_system.run_now(&self.world);
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
