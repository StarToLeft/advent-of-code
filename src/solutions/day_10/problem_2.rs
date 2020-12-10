use crate::{
    helper::{parse, prepend_slice},
    Solution,
};

pub struct Problem2 {}

impl Problem2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem2 {
    fn run(&mut self, data: &String) {
        let input = parse("\n", data);
        let mut input: Vec<usize> = input.iter().map(|s| s.parse().unwrap()).collect();
        input.sort_unstable();
        println!("Input: {:?}", input);

        let mut path: Vec<i64> = vec![];
        unsafe { prepend_slice(&mut path, &[1]) };
        unsafe { prepend_slice(&mut input, &[0]) };
        for _ in 0..input.len() - 1 {
            path.push(0);
        }

        for i in 0..input.len() {
            let streak = input[i];
            for x in 0..3 {
                let cx = i + x + 1;
                if cx <= input.len() - 1 && input[cx] - streak <= 3 {
                    path[cx] += path[i];
                }
            }
        }

        println!("{:?}", path.get(path.len()-1).unwrap());
    }

    fn test(&mut self) {}
}