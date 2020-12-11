use std::fmt::Debug;

use crate::{helper::parse, Solution};

pub struct Problem1 {}

impl Problem1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem1 {
    fn run(&mut self, data: &String) {
        let mut data = parse("\n", data);
        data.pop();
        let mut input = vec![];
        let mut sim: Vec<Vec<SeatType>> = vec![];
        for d in data {
            let mut build_line_sim = vec![];
            let mut build_line = vec![];
            for x in d.chars() {
                match x {
                    '.' => {
                        build_line.push(SeatType::F);
                        build_line_sim.push(SeatType::F);
                    }
                    'L' => {
                        build_line.push(SeatType::E);
                        build_line_sim.push(SeatType::E);
                    }
                    '#' => {
                        build_line.push(SeatType::O);
                        build_line_sim.push(SeatType::O);
                    }
                    _ => {}
                }
            }
            input.push(build_line);
            sim.push(build_line_sim);
        }

        let count_rows: i64 = input.len() as i64;
        let count_columns: i64 = input[0].len() as i64;

        let mut last_input = input.clone();
        let mut count_canges = 0;
        while count_canges < 1 {
            for x in 0..count_rows {
                for y in 0..count_columns {
                    let home: (i64, i64) = (x as i64, y as i64);
                    let seat = last_input[x as usize][y as usize];

                    let z = vec![
                        (home.0, home.1 + 1),
                        (home.0, home.1 - 1),
                        (home.0 - 1, home.1),
                        (home.0 + 1, home.1),
                        (home.0 - 1, home.1 - 1),
                        (home.0 + 1, home.1 - 1),
                        (home.0 - 1, home.1 + 1),
                        (home.0 + 1, home.1 + 1),
                    ];

                    let mut count = 0;
                    for h in z {
                        count += is_of_type(SeatType::O, &last_input, h, count_columns, count_rows);
                    }

                    if seat == SeatType::E && count == 0 {
                        sim[x as usize][y as usize] = SeatType::O;
                    } else if seat == SeatType::O && count >= 4 {
                        sim[x as usize][y as usize] = SeatType::E;
                    }
                }
            }

            if sim == last_input {
                count_canges += 1;
            }

            last_input = sim.clone();
        }

        let mut count_occupied = 0;
        for x in 0..count_rows {
            for y in 0..count_columns {
                if last_input[x as usize][y as usize] == SeatType::O {
                    count_occupied += 1;
                }
            }
        }

        println!("The count is: {:?}", count_occupied);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeatType {
    E,
    O,
    F,
}

pub fn print_vec<G: Debug>(input: &Vec<Vec<G>>) {
    for d in input {
        println!("{:?}", d);
    }
}

pub fn is_of_type(
    t: SeatType,
    input: &Vec<Vec<SeatType>>,
    pos: (i64, i64),
    count_columns: i64,
    count_rows: i64,
) -> i32 {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= count_rows || pos.1 >= count_columns {
        return 0;
    }

    if input[pos.0 as usize][pos.1 as usize] == t {
        return 1;
    }

    0
}
