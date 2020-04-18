extern crate keep_it_alive;

use ggez::conf::WindowMode;
use ggez::event::{self};
use ggez::ContextBuilder;
use keep_it_alive::GameState;

fn main() {
    // Make a Context and an EventLoop.
    let conf = WindowMode::default().dimensions(1024.0, 768.0);
    let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
        .window_mode(conf)
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = GameState::new(&mut ctx).unwrap();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
