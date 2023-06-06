#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// use crate::drawing::*;
// use piston_window::*;

const GRAVITY: f64 = -250.0; // Increase if the bird falls too slow
const FLAP_POWER: f64 = 200.0; // Increase if the bird jumps too low
const FLAP_DURATION: f64 = 0.2; // duration of upward movement after a flap, in seconds

#[derive(Debug)]
pub struct Bird {
    pub x: f64,
    pub y: f64,
    pub color: [f32; 4],
    pub velocity: f64,
    pub flap_timer: f64,
    pub acceleration: f64,
    // acceleration: f64,
    // is_jumping: bool,
}

impl Bird {
    pub fn new(x: f64, y: f64, color: [f32; 4], velocity: f64, acceleration: f64) -> Bird {
        Bird {
            x,
            y,
            color,
            velocity,
            flap_timer: 0.0,
            acceleration,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.acceleration = GRAVITY;

        // Apply acceleration to velocity
        self.velocity += self.acceleration * dt;

        // Update bird position.
        self.y -= self.velocity * dt;

        // Update flap timer and gravity.
        if self.flap_timer > 0.0 {
            // If bird is still flapping, decrease the flap timer.
            self.flap_timer -= dt;
        } else {
            // If not flapping, bird should fall downwards.
        }
    }

    pub fn flap(&mut self) {
        self.velocity = FLAP_POWER; // start moving upwards
        self.acceleration = 0.0;
        self.flap_timer = FLAP_DURATION; // set the flap timer
    }
}
