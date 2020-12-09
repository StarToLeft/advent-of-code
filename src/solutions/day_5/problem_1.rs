use crate::{Solution, helper::{lower_half, parse, upper_half}};

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let inputs = parse("\n", data);

        let mut seats = vec![];
        for seat in inputs {
            let mut fb_range = (0, 127);
            let mut cl_range = (0, 7);
            for l in seat.chars() {
                match l {
                    'F' => fb_range = lower_half(fb_range.0 as f32, fb_range.1 as f32),
                    'B' => fb_range = upper_half(fb_range.0 as f32, fb_range.1 as f32),
                    'R' => cl_range = upper_half(cl_range.0 as f32, cl_range.1 as f32),
                    'L' => cl_range = lower_half(cl_range.0 as f32, cl_range.1 as f32),
                    _ => {}
                }
            }
            seats.push(fb_range.0 * 8 + cl_range.0);
        }

        println!("Highest seat id {:?}", seats.iter().max().unwrap());
    }

    fn test(&mut self) {}
}