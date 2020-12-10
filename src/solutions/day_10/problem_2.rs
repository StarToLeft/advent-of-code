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

pub fn get_differences(input: &mut Vec<i64>) -> Vec<i64> {
    // ! Don't you dare remove this sort, it literally ducks it all up!
    input.sort();

    let mut differences = vec![];
    let mut last = 0;
    for i in 0..input.len() {
        let difference = input[i] - last;
        differences.push(difference);
        last = input[i];
    }

    differences
}

pub fn brute_force(differences: &Vec<i64>, total: i64) -> Vec<(i64, i64, i64)> {
    let mut solutions = vec![];

    let mut plausible = vec![];
    for x in 1..15 {
        for y in 1..15 {
            for z in 1..15 {
                plausible.push((x, z, y));
                let t = test_total(differences, x, y, z);
                if t == total {
                    solutions.push((x, y, z));
                }
            }
        }
    }

    solutions
}

pub fn test_total(differences: &Vec<i64>, x: i64, y: i64, z: i64) -> i64 {
    let mut total = 1;
    let mut streak = 0;
    let mut total_streaks = vec![];
    for i in 0..differences.len() - 1 {
        if differences[i] == 1 && differences[i + 1] == 1 {
            streak += 1;
        } else {
            match streak {
                1 => {
                    total *= 2;
                }
                2 => {
                    total *= 4;
                }
                3 => {
                    total *= 7;
                }
                _ => {
                    total *= 1;
                }
            }

            streak = 0;
        }
        total_streaks.push(streak);
    }

    total
}
