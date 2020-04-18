mod meshes;

use ggez::event::EventHandler;
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use specs::prelude::*;
use specs::{Component, VecStorage};

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
        let mut world = World::new();
        world.register::<Position>();
        world.register::<ObjectMesh>();

        world
            .create_entity()
            .with(Position {
                x: arena_width / 2.0,
                y: arena_height - 25.0,
            })
            .with(ObjectMesh(egg_mesh))
            .build();

        world
            .create_entity()
            .with(Position {
                x: 100.0,
                y: arena_height - player_width,
            })
            .with(ObjectMesh(player))
            .build();
        Ok(GameState { world })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
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

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct ObjectMesh(Mesh);

struct RenderSystem<'a> {
    context: &'a mut Context,
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, ObjectMesh>);

    fn run(&mut self, (position, mesh): Self::SystemData) {
        use specs::Join;

        for (position, mesh) in (&position, &mesh).join() {
            graphics::draw(
                self.context,
                &mesh.0,
                graphics::DrawParam::default().dest(Point2::new(position.x, position.y)),
            )
            .unwrap();
        }
    }
}
