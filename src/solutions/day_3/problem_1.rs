use crate::Solution;
use std::iter::FromIterator;

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let data = data.split("\n");
        let mut inputs = vec![];
        for d in data {
            inputs.push(d.to_string().replace("\r", ""));
        }

        let mut trees = 0;
        let mut right = 0;
        let mut lock = 0;
        for i in inputs.iter_mut() {
            if lock == 0 {
                lock = 1;
            }

            let mut chars: Vec<char> = i.chars().collect();

            while right >= chars.len() {
                let mut c = i.to_owned().chars().collect();
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

            right += 3;
        }

        println!("Counted trees: {:?}", trees);
    }
}
