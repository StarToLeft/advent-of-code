use crate::helper::*;
use crate::Solution;

pub struct Problem2 {}

impl Problem2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem2 {
    fn run(&mut self, data: &String) {
        let mut input = parse("\n", data);
        input.pop();

        let jumps = count_string(&input, "jmp");
        let nops = count_string(&input, "nop");

        let mut t = jumps;
        for _ in 0..2 {
            for i in 0..t {
                let mut copy_input = input.clone();
                replace_local_index_string(&mut copy_input, "jmp", i, "nop");
                let result = try_stack(&copy_input);
                if !result.0 {
                    println!("Found the error: {}", result.1);
                    return;
                }
            }

            t = nops;
        }
    }
}

pub fn try_stack(input: &Vec<String>) -> (bool, i32) {
    let mut acc: i32 = 0;
    let mut index: i32 = 0;
    let mut has_run = vec![];
    while (index as usize) < input.len() {
        let line = &input[index as usize];
        if has_run.contains(&index) {
            return (true, acc);
        }
        let function = &line[0..3];

        let nx = line.substring(5, 12);
        let mut number = nx.parse::<i32>().unwrap();
        let negative_positive = line.chars().nth(4).unwrap();
        if negative_positive == '-' {
            number *= -1;
        }

        has_run.push(index);
        index += 1;
        match function {
            "nop" => {}
            "acc" => {
                acc += number;
            }
            "jmp" => {
                index = index + number - 1;
            }
            _ => {}
        }
    }

    return (false, acc);
}
