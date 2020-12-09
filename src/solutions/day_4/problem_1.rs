use crate::{helper::parse, Solution};
pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let mut inputs = parse("\n\n", data);

        let mut count = 0;
        for i in 0..inputs.len() {
            let input = inputs.get_mut(i).unwrap();
            let fields_str = input.replace("\n", " ");
            let fields = fields_str.split(" ").collect::<Vec<_>>();

            let mut fields_checked: Vec<String> = vec![];
            for f in fields {
                if f.contains("byr") {
                    fields_checked.push("byr".to_string());
                } else if f.contains("iyr") {
                    fields_checked.push("iyr".to_string());
                } else if f.contains("eyr") {
                    fields_checked.push("eyr".to_string());
                } else if f.contains("hgt") {
                    fields_checked.push("hgt".to_string());
                } else if f.contains("hcl") {
                    fields_checked.push("hcl".to_string());
                } else if f.contains("ecl") {
                    fields_checked.push("ecl".to_string());
                } else if f.contains("pid") {
                    fields_checked.push("pid".to_string());
                }
            }

            fields_checked.sort();
            if fields_checked == ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"] {
                count += 1;
            }
        }

        println!("Counted {:?} passports", count);
    }

    fn test(&mut self) {}
}

