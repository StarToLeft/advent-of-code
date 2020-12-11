use std::convert::TryInto;

use crate::{Solution, helper::{find_sub, parse}};

pub struct Problem2 {}

impl Problem2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem2 {
    fn run(&mut self, data: &String) {
        let input = parse("\n", data);

        let mut previous = vec![];
        let current_preamble = 25;
        let mut invalid = 0;
        for i in 0..input.len() {
            if i <= current_preamble {
                previous.push(input[i].clone());
                continue;
            }

            let r_input = input[i].clone();
            let r_input = r_input.parse::<i64>().unwrap();

            let mut found = false;
            for z in &previous {
                let z = z.parse::<i64>().unwrap();
                for x in &previous {
                    let x = x.parse::<i64>().unwrap();
                    if (z + x) == r_input {
                        found = true;
                    }
                }
            }

            if found == true {
                previous = vec![];
                let calc = (i as i32) - 1;
                for i in 0..current_preamble {
                    previous.push(input[(calc - ((i) as i32 - 1)) as usize].clone());
                }
            } else {
                invalid = r_input;
                break;
            }
        }

        println!("Invalid {}", invalid);

        let mut inp = vec![];
        for i in &input {
            if i == "" { continue; }
            let x = i.parse::<i64>().unwrap();
            inp.push(x);
        }

        let solved = find_sub(&inp, inp.len().try_into().unwrap(), invalid);

        let mut sum = 0;
        let mut x = vec![];
        for i in 0..inp.len() {
            if i >= solved.1[0].0.try_into().unwrap() && i <= solved.1[0].1.try_into().unwrap() {
                x.push(inp[i]);
                sum += inp[i];
            }
        }
        x.sort();
        assert_eq!(invalid, sum);
        println!("Sum of values {:?}", x[0] + x[x.len() - 1]);
    }
}
