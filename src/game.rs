// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use piston_window::*;

use crate::bird::Bird;
use crate::drawing::*;
use crate::obstacles::Pipe;

const PLAYER_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const PIPE_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const PIPE_SPEED: f64 = 80.0;
const PIPE_FREQUENCY: f64 = 3.0; // Increase to spawn pipes more frequently, in seconds

pub struct Game {
    bird: Bird,
    pipes: Vec<Pipe>,
    pipe_spawn_timer: f64,
    window_width: f64,
    window_height: f64,
}

impl Game {
    pub fn new(window_width: f64, window_height: f64) -> Game {
        Game {
            bird: Bird {
                x: window_width / 2.0,
                y: window_height / 2.0,
                color: PLAYER_COLOR,
                velocity: 0.0,
                flap_timer: 0.0,
                acceleration: 0.0,
            },
            pipes: Vec::new(),
            pipe_spawn_timer: 0.0,
            window_width,
            window_height,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if let Key::Space = key {
            self.bird.flap();
        }

        println!("bird: {:?}", self.bird);
        println!("pipes: {:?}", self.pipes);
    }

    fn bird_hits_pipe(&self) -> bool {
        for pipe in &self.pipes {
            if (self.bird.x < pipe.x + 20.0 && self.bird.x > pipe.x - 20.0)
                && (self.bird.y < pipe.y - pipe.gap_height / 2.0
                    || self.bird.y > pipe.y + pipe.gap_height / 2.0)
            {
                return true;
            }
        }
        false
    }

    fn bird_out_of_bounds(&self) -> bool {
        for pipe in &self.pipes {
            if self.bird.y < 0.0 || self.bird.y > self.window_height {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self, dt: f64) {
        // Update bird position.

        self.bird.update(dt);

        // Update the pipes - move them to the left & remove offscreen ones.
        for pipe in &mut self.pipes {
            pipe.x -= PIPE_SPEED * dt;
        }

        // Keep pipes that are still onscreen.
        self.pipes.retain(|pipe| pipe.x > 0.0);

        // Spawn new pipes.
        self.pipe_spawn_timer -= dt;

        if self.pipe_spawn_timer <= 0.0 {
            self.pipe_spawn_timer += PIPE_FREQUENCY;

            // Randomly generate a gap.
            let (gap_y, gap_height) = Pipe::generate_random_gap(
                self.window_height as i32,
                (self.window_height as f64 / 4.5) as i32,
                self.window_height as i32 / 3,
            );

            let new_pipe = Pipe::new(self.window_width, gap_y as f64, gap_height as f64);
            self.pipes.push(new_pipe);
        }

        if self.bird_hits_pipe() || self.bird_out_of_bounds() {
            self.reset();
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);

        // Draw the bird.
        rectangle(
            self.bird.color,
            [self.bird.x - 10.0, self.bird.y - 10.0, 20.0, 20.0],
            con.transform,
            g,
        );

        // Draw the pipes.
        for pipe in &self.pipes {
            rectangle(
                PIPE_COLOR,
                [pipe.x - 10.0, 0.0, 40.0, pipe.y - pipe.gap_height / 2.0],
                con.transform,
                g,
            );
            rectangle(
                PIPE_COLOR,
                [
                    pipe.x - 10.0,
                    pipe.y + pipe.gap_height / 2.0,
                    40.0,
                    self.window_height - (pipe.y + pipe.gap_height / 2.0),
                ],
                con.transform,
                g,
            );
        }
    }

    fn reset(&mut self) {
        self.bird = Bird::new(self.window_width / 2.0, self.window_height / 2.0, PLAYER_COLOR, 0.0, 0.0);
        self.pipes = Vec::new();
        self.pipe_spawn_timer = 0.0;
    }
}
