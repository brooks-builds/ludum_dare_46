mod components;
mod meshes;
mod systems;

use components::{
    Acceleration, Drag, Floor, HasGravity, Height, ObjectMesh, OnGround, Position, Velocity,
};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use specs::prelude::*;
use systems::{
    ApplyForceSystem, DragSystem, GravitySystem, HitGround, MovePlayerSystem, RenderSystem,
};

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
        world.register::<OnGround>();

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

        gravity_system.run_now(&self.world);
        hit_ground.run_now(&self.world);
        move_system.run_now(&self.world);
        move_player_system.run_now(&self.world);
        drag_system.run_now(&self.world);
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
