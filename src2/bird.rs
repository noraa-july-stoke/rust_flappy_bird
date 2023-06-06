#![allow(dead_code)]
use crate::drawing::*;
use piston_window::*;

#[derive(Debug)]

pub struct Bird {
    pub x: i32,
    pub y: i32,
    pub velocity: i32,
    pub acceleration: i32,
    pub is_jumping: bool,
}

impl Bird {
    pub fn new(x: i32, y: i32, velocity: i32, acceleration: i32) -> Bird {
        Bird {
            x,
            y,
            velocity,
            acceleration,
            is_jumping: false,
        }
    }

    pub fn jump(&mut self) {
        self.velocity = -12; // Adjust this value for desired jump height
        self.is_jumping = true;
    }

    pub fn update(&mut self) {
        // Apply gravity to the bird's velocity
        self.velocity += self.acceleration;

        // Limit the maximum velocity to prevent the bird from falling too fast
        const MAX_VELOCITY: i32 = 10;
        if self.velocity > MAX_VELOCITY {
            self.velocity = MAX_VELOCITY;
        }

        // Update the bird's position based on its velocity
        self.y += self.velocity;
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let gui_x = to_gui_coord(self.x);
        let gui_y = to_gui_coord(self.y);

        rectangle(
            [0.5, 0.0, 0.5, 1.0],
            [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
            con.transform,
            g,
        );
    }
}
