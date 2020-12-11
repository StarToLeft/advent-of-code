use crate::{Solution, helper::parse};

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem1 {
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
    }
}
