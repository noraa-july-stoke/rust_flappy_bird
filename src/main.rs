#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod bird;
mod drawing;
mod game;
mod obstacles;

use piston_window::*;

fn main() {
    let (width, height) = (800, 800);

    let mut window: PistonWindow = WindowSettings::new("Rust Flappy", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new(width as f64, height as f64);
    let mut events = Events::new(EventSettings::new()).ups(60);

    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&e, |c, g, _| {
            game.draw(&c, g);
        });

        e.update(|arg| {
            game.update(arg.dt);
        });
    }
}
