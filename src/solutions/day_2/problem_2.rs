use crate::Solution;

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
        for i in data {
            if i == "" { continue; }
            inputs.push(i);
        }

        let mut valid = 0;
        for p in inputs {
            let specs = p.split(" ").collect::<Vec<&str>>()[0];
            let min = specs.split("-").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
            let max = specs.split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

            let ch = p.split(" ").collect::<Vec<&str>>()[1].replace(":", "").parse::<char>().unwrap();
            let pass = p.split(" ").collect::<Vec<&str>>()[2] as &str;

            let char_min = pass.chars().collect::<Vec<char>>()[min as usize - 1];
            let char_max = pass.chars().collect::<Vec<char>>()[max as usize - 1];

            if char_min == ch && char_max != ch || char_min != ch && char_max == ch {
                valid += 1;
            }
        }

        println!("{:?}", valid);
    }
}