use regex::Regex;

use crate::{Solution, helper::parse};

pub struct Problem2 {}

impl Problem2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem2 {
    fn run(&mut self, data: &String) {
        let mut inputs = parse("\n\n", data);

        let cm_reg = Regex::new(r"^\d+cm$").unwrap();
        let in_reg = Regex::new(r"^\d+in$").unwrap();
        let hex_reg = Regex::new(r"^#[\dA-Fa-f]{6}$").unwrap();
        let pid_reg = Regex::new(r"^[0-9]{9}$").unwrap();

        let mut count = 0;
        for i in 0..inputs.len() {
            let input = inputs.get_mut(i).unwrap();
            let fields_str = input.replace("\n", " ");
            let fields = fields_str.split(" ").collect::<Vec<_>>();

            let mut fields_checked: Vec<String> = vec![];
            for f in fields {
                if f.contains("byr") {
                    let val = f.split(":").collect::<Vec<_>>()[1];
                    let x = val.parse::<i32>().unwrap();
                    if x >= 1920 && x <= 2002 {
                        fields_checked.push("byr".to_string());
                    }
                } else if f.contains("iyr") {
                    let val = f.split(":").collect::<Vec<_>>()[1];
                    let x = val.parse::<i32>().unwrap();
                    if x >= 2010 && x <= 2020 {
                        fields_checked.push("iyr".to_string());
                    }
                } else if f.contains("eyr") {
                    let val = f.split(":").collect::<Vec<_>>()[1];
                    let x = val.parse::<i32>().unwrap();
                    if x >= 2020 && x <= 2030 {
                        fields_checked.push("eyr".to_string());
                    }
                } else if f.contains("hgt") {
                    let val = f.split(":").collect::<Vec<_>>()[1];
                    if val.contains("in") || val.contains("cm") {
                        if cm_reg.is_match(val) {
                            let x = val.replace("cm", "").parse::<i32>().unwrap();
                            if x >= 150 && x <= 193 {
                                fields_checked.push("hgt".to_string());
                            }
                        } else {
                            if in_reg.is_match(val) {
                                let x = val.replace("in", "").parse::<i32>().unwrap();
                                if x >= 59 && x <= 76 {
                                    fields_checked.push("hgt".to_string());
                                }
                            }
                        }
                    }
                } else if f.contains("hcl") {
                    let val = f.split(":").collect::<Vec<_>>()[1];
                    if hex_reg.is_match(val) {
                        fields_checked.push("hcl".to_string());
                    }
                } else if f.contains("ecl") {
                    let val = f.split(":").collect::<Vec<_>>()[1];
                    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val) {
                        fields_checked.push("ecl".to_string());
                    }
                } else if f.contains("pid") {
                    let val = f.split(":").collect::<Vec<_>>()[1];
                    if pid_reg.is_match(val) {
                        fields_checked.push("pid".to_string());
                    }
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
