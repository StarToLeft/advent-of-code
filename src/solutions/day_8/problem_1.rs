use crate::helper::*;
use crate::Solution;

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let input = parse("\n", data);
        println!("Accumulator: {}", try_stack(&input).1);
    }

    fn test(&mut self) {}
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