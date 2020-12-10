use std::convert::TryInto;

use crate::{Solution, helper::parse};

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let data = parse("\n", data);
        let mut input = vec![];
        for d in data {
            let x = d.parse::<u64>().unwrap();
            input.push(x);
        }
        input.sort();

        let mut one_jolt_differences = 1;
        let mut three_jolt_differences = 1;

        let mut highest_outlet: u64 = 0;
        let mut last_highest_outlet: u64 = 0;
        for mut i in 0..input.len() {
            let outlet = input[i];

            // println!("Outlet: {}", outlet);

            let mut higher = 0;
            for _ in 0..3 {
                let mut found_higher = false;
                for y in i+1..input.len() {
                    if input[y]-outlet <= 3 && input[y]-outlet >= 1 {
                        higher = y;
                        found_higher = true;

                        if input[y]-outlet == 1 {
                            one_jolt_differences += 1;
                        } else if input[y]-outlet == 3 {
                            three_jolt_differences += 1;
                        }
                        break;
                    } 
                }

                if found_higher {
                    break;
                }
            }

            if higher > highest_outlet.try_into().unwrap() {
                // highest_outlet = i;
            } else if highest_outlet == last_highest_outlet {
                highest_outlet = outlet;
                break;
            }

            last_highest_outlet = highest_outlet;
        }

        println!("Highest outlet found {}", highest_outlet);
        println!("Jolt differences {} | {}", one_jolt_differences, three_jolt_differences);
        println!("Multiplied {}", one_jolt_differences * three_jolt_differences);
    }

    fn test(&mut self) {

    }
}