#![allow(dead_code)]
use crate::bird::Bird;
use crate::obstacles::Pipe;
use piston_window::*;
use rand::Rng;

#[derive(Debug)]
enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    bird: Bird,
    game_state: GameState,
    pipes: Vec<Pipe>,
    window_size: (i32, i32),
    score: u32,
}

impl Game {
    pub fn new(window_size: (i32, i32)) -> Game {
        Game {
            bird: Bird::new(window_size.0 / 2, window_size.1 / 2, 0, 2),
            game_state: GameState::Playing,
            pipes: vec![Pipe::new(
                window_size.0,
                window_size.1 / 2,
                window_size.1 / 4,
            )],
            window_size,
            score: 0,
        }
    }

    pub fn draw(&mut self, con: &Context, g: &mut G2d) {
        match self.game_state {
            GameState::Playing => {
                // Drawing Bird
                rectangle(
                    [0.5, 0.0, 0.5, 1.0],                               // purple color
                    [self.bird.x as f64, self.bird.y as f64, 1.0, 1.0], // dimensions
                    con.transform,
                    g,
                );

                // Drawing Pipes
                for pipe in &self.pipes {
                    rectangle(
                        [1.0, 0.0, 0.0, 1.0], // red color
                        [
                            pipe.x as f64,
                            0.0,
                            1.0,
                            (pipe.gap_y - pipe.gap_height / 2) as f64,
                        ], // upper pipe
                        con.transform,
                        g,
                    );
                    rectangle(
                        [1.0, 0.0, 0.0, 1.0], // red color
                        [
                            pipe.x as f64,
                            (pipe.gap_y + pipe.gap_height / 2) as f64,
                            1.0,
                            (self.window_size.1 - (pipe.gap_y + pipe.gap_height / 2)) as f64,
                        ], // lower pipe
                        con.transform,
                        g,
                    );
                }
            }
            GameState::GameOver => {
                // Drawing code for GameOver state could go here
            }
        }
    }

    pub fn on_key_press(&mut self, key: Key) {
        match self.game_state {
            GameState::Playing => {
                if key == Key::Space {
                    // This will cause the bird to "flap" (move upwards)
                    self.bird.velocity = -8;
                }
            }
            GameState::GameOver => {
                if key == Key::R {
                    // This will reset the game
                    *self = Game::new(self.window_size);
                }
            }
        }
    }

    fn generate_random_gap(
        window_height: i32,
        min_gap_height: i32,
        max_gap_height: i32,
    ) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        let gap_height = rng.gen_range(min_gap_height..=max_gap_height);
        let gap_y = rng.gen_range(50..=window_height - 50 - gap_height);
        println!("gap_y: {}, gap_height: {}", gap_y, gap_height);
        (gap_y, gap_height)
    }

    pub fn update(&mut self) {
        match self.game_state {
            GameState::Playing => {
                // Update bird's position and velocity
                self.bird.update();

                // Check for collision with pipes
                for pipe in &mut self.pipes {
                    if pipe.check_collision_with_bird(&self.bird) {
                        self.game_state = GameState::GameOver;
                        break;
                    }
                }

                // Move and possibly delete pipes
                let mut i = 0;
                while i != self.pipes.len() {
                    self.pipes[i].move_pipe();

                    // if pipe is out of screen
                    if self.pipes[i].is_offscreen() {
                        self.pipes.remove(i);
                    } else {
                        i += 1;
                    }
                }

                // Possibly add a new pipe
                if let Some(last_pipe) = self.pipes.last() {
                    if last_pipe.x < self.window_size.0 - 100 {
                        println!("bird y: {}", self.bird.y);
                        println!("bird: {:?}", self.bird);
                        let (random_gap_y, random_gap_height) =
                            Game::generate_random_gap(self.window_size.1, 50, 200);
                        self.pipes.push(Pipe::new(
                            self.window_size.0,
                            random_gap_y,
                            random_gap_height,
                        ));
                    }
                }

                if let Some(first_pipe) = self.pipes.first() {
                    if first_pipe.x as f64 + 1.0 < self.bird.x as f64 {
                        self.score += 1;
                        self.pipes.remove(0);
                    }
                }
            }
            GameState::GameOver => {
                // Do nothing
            }
        }
    }

    pub fn reset(&mut self) {
        self.bird = Bird::new(self.window_size.0 / 2, self.window_size.1 / 2, 0, 0);
        self.game_state = GameState::Playing;
        self.pipes = vec![];
        self.score = 0;
    }

    // The main game loop
    pub fn run(&mut self, window: &mut PistonWindow) {
        while let Some(event) = window.next() {
            if let Some(Button::Keyboard(key)) = event.press_args() {
                if key == Key::Space {
                    match self.game_state {
                        GameState::Playing => self.bird.jump(),
                        GameState::GameOver => self.reset(),
                    }
                }
            }

            window.draw_2d(&event, |c, g, _| {
                clear([1.0; 4], g);
                self.draw(&c, g);
            });

            if let Some(_) = event.update_args() {
                self.update();
            }
        }
    }
}
