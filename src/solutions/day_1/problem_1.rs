use crate::Solution;

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let data = data.split("\n");
        let mut inputs = vec![];
        for d in data {
            if d == "" { continue; }
            inputs.push(d.parse::<i32>().unwrap());
        }

        for i in 0..inputs.len() {
            for x in 0..inputs.len() {
                if inputs[i] + inputs[x] == 2020 {
                    println!("{:?}", inputs[i] * inputs[x]);
                    return;
                }
            }
        }
    }

    fn test(&mut self) {

    }
}