use crate::Solution;

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
        for i in data {
            if i == "" { continue; }
            inputs.push(i);
        }

        let mut valid = 0;
        for p in inputs {
            let specs = p.split(" ").collect::<Vec<&str>>()[0];
            let min = specs.split("-").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
            let max = specs.split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

            let ch = p.split(" ").collect::<Vec<&str>>()[1].replace(":", "");
            let pass = p.split(" ").collect::<Vec<&str>>()[2];

            if count_chars(pass, ch.clone()) as usize >= min && count_chars(pass, ch.clone()) as usize <= max {
                valid += 1;
            }
        }

        println!("{:?}", valid);
    }
} 

fn count_chars(d: &str, c: String) -> u32 {
    let mut o = 0;
    for d in d.chars() {
        if d.to_string() == c {
            o += 1;
        }
    }
    return o;
}