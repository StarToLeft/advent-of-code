use std::collections::HashMap;

use crate::{helper::parse, Solution};

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let raw_inputs = parse(".", data);
        let mut inputs = vec![];
        for raw_in in raw_inputs {
            let raw_in = raw_in.replace("\n", "");
            inputs.push(raw_in);
        }

        let mut color_rules: HashMap<String, Color> = HashMap::new();
        for input in &inputs {
            let raw_inputs = parse(" ", input);

            if raw_inputs.len() < 7 {
                continue;
            }

            let name = raw_inputs[0].clone() + " " + &raw_inputs[1];

            let color = Color::new(name, input.to_string());
            color_rules.insert(color.name.clone(), color);
        }

        for color in color_rules.iter_mut() {
            let color = color.1;
            let raw_inputs = parse(",", &color.full);

            let mut contains = vec![];

            let mut index = 0;
            for inp in raw_inputs {
                let l_input = parse(" ", &inp);
                if l_input.len() < 4 {
                    continue;
                }

                let mut child = ChildColor::new(String::new(), inp, 0, false);
                if index == 0 {
                    if l_input[4].contains("no") {
                        child.is_end = true;
                    } else {
                        child.amount = l_input[4].parse::<u32>().unwrap();
                        child.name = l_input[5].clone() + " " + &l_input[6];
                    }
                } else {
                    let mut index = 0;
                    if l_input[0] == " " || l_input[0].chars().all(char::is_numeric) {
                        index += 2;
                    }

                    child.name = l_input[index].clone() + " " + &l_input[index + 1];
                }

                if child.name == "" {
                    child.name = l_input[0].clone() + " " + &l_input[1];
                }

                contains.push(child);
                index += 1;
            }

            color.contains = contains;
            color.full = String::new();
        }

        let shiny_gold = color_rules.get("shiny gold").unwrap().clone();
        let mut has_done = vec![];
        let count = traverse_main(shiny_gold, &color_rules, &mut 0, &mut has_done);

        println!("Bag colors {:?}", count);
    }

    fn test(&mut self) {}
}

pub fn traverse_main(
    color: Color,
    hash: &HashMap<String, Color>,
    count: &mut u32,
    has_done: &mut Vec<String>
) -> u32 {
    let mut directly_holders = vec![];
    for (_, c) in hash {
        for ch in &c.contains {
            if ch.name == color.name {
                directly_holders.push(c);
            }
        }
    }

    for direct in directly_holders {
        if direct.name != color.name && !has_done.contains(&direct.name) {
            for ch in &direct.contains {
                if ch.name == color.name {
                    *count += 1;
                }
            }

            traverse_main(direct.to_owned(), hash, count, has_done);
            has_done.push(direct.to_owned().name);
        }
    }

    count.to_owned()
}

#[derive(Debug, Clone)]
pub struct Color {
    full: String,
    name: String,
    contains: Vec<ChildColor>,
}

impl Color {
    pub fn new(name: String, full: String) -> Self {
        Self {
            name,
            contains: vec![],
            full,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChildColor {
    name: String,
    amount: u32,
    is_end: bool,
}

impl ChildColor {
    pub fn new(name: String, _: String, amount: u32, is_end: bool) -> Self {
        Self {
            name,
            amount,
            is_end,
        }
    }
}
