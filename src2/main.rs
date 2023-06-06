

mod bird;
mod obstacles;
mod game;
mod drawing;
mod game_debug;

use piston_window::*;
use game_debug::Game;

fn main() {
    let (width, height) = (640, 480);

    let mut window: PistonWindow = WindowSettings::new("Flappy Bird", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new((width as i32, height as i32));

    game.run(&mut window);
}
