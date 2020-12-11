use crate::{helper::parse, Solution};

pub struct Problem2 {}

impl Problem2 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
struct Group {
    people: Vec<Person>,
}

#[derive(Clone, Debug)]
struct Person {
    questions_correct: Vec<String>,
}

impl Solution for Problem2 {
    fn run(&mut self, data: &String) {
        let mut raw_inputs = parse("\n", data);
        raw_inputs.pop();

        let mut abc_arr = "abcdefghijklmnopqrstuvwxyz".split("").collect::<Vec<_>>();
        abc_arr.sort();

        let mut groups = vec![];
        let mut cache_group = Group { people: vec![] };
        for i in raw_inputs.iter() {
            if i == "" {
                groups.push(cache_group);
                cache_group = Group { people: vec![] };
                continue;
            } else {
                let is = i.split("");
                let mut person = Person {
                    questions_correct: vec![],
                };
                for il in is {
                    if il == "" || il == "\n" && !abc_arr.contains(&il) {
                        continue;
                    }

                    person.questions_correct.push(il.to_string());
                }

                cache_group.people.push(person);
            }
        }
        groups.push(cache_group);

        let mut ex_sum = 0;
        for group in groups {
            for abc_check in &abc_arr {
                if abc_check == &"" {
                    continue;
                }
                let mut contains = true;
                for person in &group.people {
                    if !person.questions_correct.contains(&abc_check.to_string()) {
                        contains = false;
                    }
                }
                if contains {
                    ex_sum += 1;
                }
            }
        }

        println!("Counted {:?} correct in all groups", ex_sum);
    }
}
