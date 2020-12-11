use crate::Solution;
use std::iter::FromIterator;

pub struct Problem2 {}

impl Problem2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem2 {
    fn run(&mut self, data: &String) {
        let data = data.split("\n");
        let mut inputs = vec![];
        for d in data {
            inputs.push(d.to_string().replace("\r", ""));
        }

        let path_definers = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut result = vec![];

        for path in path_definers {
            let mut local_inputs = inputs.clone();
            let mut trees = 0;
            let mut right = 0;
            let mut lock = 0;
            for i in local_inputs.iter_mut().step_by(path.1) {
                if lock == 0 {
                    lock = 1;
                }

                let mut chars: Vec<char> = i.chars().collect();
                while right >= chars.len() {
                    let mut c = i.chars().collect();
                    chars.append(&mut c);
                }

                if chars[right] == '#' && lock != 0 {
                    chars[right] = 'X';
                    trees += 1;
                }

                if chars[right] == '.' && lock != 0 {
                    chars[right] = 'O';
                }

                let s = String::from_iter(chars);
                *i = s;

                right += path.0 as usize;
            }

            result.push(trees);
        }

        println!("{:?}", result)
    }
}
