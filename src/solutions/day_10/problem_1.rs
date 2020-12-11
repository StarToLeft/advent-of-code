use crate::{helper::parse, Solution};

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let input = parse("\n", data);
        let mut input: Vec<usize> = input.iter().map(|s| s.parse().unwrap()).collect();
        input.sort_unstable();

        let mut scores = (1, 1);
        for i in 0..input.len() {
            for _ in 0..3 {
                let mut found_higher = false;
                for y in i + 1..input.len() {
                    found_higher = true;
                    if input[y] - input[i] == 1 {
                        scores.0 += 1;
                    } else if input[y] - input[i] == 3 {
                        scores.1 += 1;
                    }
                    break;
                }

                if found_higher {
                    break;
                }
            }
        }

        println!("Jolt differences {:?}", scores);
        println!("Multiplied {}", scores.0 * scores.1);
    }
}