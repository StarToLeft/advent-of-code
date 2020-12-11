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
        for d in data {
            if d == "" {
                continue;
            }
            inputs.push(d.parse::<i32>().unwrap());
        }

        for y in 0..inputs.len() {
            for i in 0..inputs.len() {
                for x in 0..inputs.len() {
                    if inputs[i] + inputs[x] + inputs[y] == 2020 {
                        println!("{:?}", inputs[i] * inputs[x] * inputs[y]);
                        return;
                    }
                }
            }
        }
    }
}
