#![allow(dead_code)]
use crate::bird::Bird;
use crate::drawing::*;
use piston_window::*;

pub struct Pipe {
    pub x: i32,
    pub gap_y: i32,      // The y-coordinate for the center of the gap.
    pub gap_height: i32, // The height of the gap.
}

impl Pipe {
    pub fn new(x: i32, gap_y: i32, gap_height: i32) -> Pipe {
        Pipe {
            x,
            gap_y,
            gap_height,
        }
    }

    pub fn draw(&self, window_size: (i32, i32), con: &Context, g: &mut G2d) {
        let pipe_x = to_gui_coord(self.x);
        let _pipe_y_upper = to_gui_coord(self.gap_y - self.gap_height / 2);
        let pipe_y_lower = to_gui_coord(self.gap_y + self.gap_height / 2);
        let pipe_height_upper = to_gui_coord(self.gap_y - self.gap_height / 2);
        let pipe_height_lower = to_gui_coord(window_size.1 - (self.gap_y + self.gap_height / 2));

        rectangle(
            [1.0, 0.0, 0.0, 1.0], // Red color for pipes
            [pipe_x, 0.0, BLOCK_SIZE, pipe_height_upper],
            con.transform,
            g,
        );
        rectangle(
            [1.0, 0.0, 0.0, 1.0], // Red color for pipes
            [pipe_x, pipe_y_lower, BLOCK_SIZE, pipe_height_lower],
            con.transform,
            g,
        );
    }

    pub fn check_collision_with_bird(&self, bird: &Bird) -> bool {
        let bird_x = bird.x as f64;
        let bird_y = bird.y as f64;

        let pipe_x = self.x as f64;

        let pipe_y_upper = self.gap_y as f64 - self.gap_height as f64 / 2.0;
        let pipe_y_lower: f64 = self.gap_y as f64 + self.gap_height as f64 / 2.0;

        // Checks if the bird is within the pipe's x-coordinates and
        // outside of the gap's y-coordinates & returns the result as bool.
        bird_x > pipe_x && bird_x < pipe_x + 1.0 && (bird_y < pipe_y_upper || bird_y > pipe_y_lower)
    }

    pub fn move_pipe(&mut self) {
        self.x -= 1;
    }

    pub fn is_offscreen(&self) -> bool {
        self.x < 0
    }

    // Include other methods for game mechanics, such as moving the pipe...
}
