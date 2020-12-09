use crate::{helper::parse, Solution};

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let mut raw_inputs = parse("\n", data);
        raw_inputs.pop();

        let mut abc_arr = "abcdefghijklmnopqrstuvwxyz".split("").collect::<Vec<_>>();
        abc_arr.sort();

        let mut inputs = vec![];
        let mut built_group = String::new();
        for i in raw_inputs.iter() {
            if i == "" {
                inputs.push(built_group.clone());
                built_group = String::new();
                continue;
            }

            built_group += &i.clone();
        }
        inputs.push(built_group.clone());

        let mut sum = 0;
        for c in inputs {
            let mut checked = vec![];
            let mut failed = false;
            for c in c.chars() {
                if !checked.contains(&c) && abc_arr.contains(&c.to_string().as_str()) {
                    checked.push(c);
                } else if !abc_arr.contains(&c.to_string().as_str()) {
                    failed = true;
                }
            }
            if failed {
                continue;
            }
            checked.sort();
            sum += checked.len();
        }

        println!("All groups sum equaled {:?}", sum);
    }

    fn test(&mut self) {}
}
