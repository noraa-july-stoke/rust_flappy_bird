#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::Rng;

#[derive(Debug)]
pub struct Pipe {
    pub x: f64,
    pub y: f64,          // This is the y-coordinate for the center of the gap.
    pub gap_height: f64, // The height of the gap.
}

impl Pipe {
    pub fn new(x: f64, y: f64, gap_height: f64) -> Pipe {
        Pipe { x, y, gap_height }
    }
    pub fn generate_random_gap(
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
}
