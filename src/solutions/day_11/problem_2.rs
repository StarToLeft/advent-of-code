use std::fmt::Debug;

use crate::{helper::parse, Solution};

pub struct Problem2 {}

impl Problem2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem2 {
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
        let count_canges = 0;
        while count_canges < 1 {
            for x in 0..count_rows {
                for y in 0..count_columns {
                    let count = count_rays(&last_input, (x, y), count_columns, count_rows);

                    if last_input[x as usize][y as usize] == SeatType::E && count == 0 {
                        sim[x as usize][y as usize] = SeatType::O;
                    } else if last_input[x as usize][y as usize] == SeatType::O && count >= 5 {
                        sim[x as usize][y as usize] = SeatType::E;
                    }
                }
            }

            if sim == last_input {
                break;
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

        println!("Count: {}", count_occupied);
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
) -> i64 {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= count_rows || pos.1 >= count_columns {
        return 0;
    }

    if input[pos.0 as usize][pos.1 as usize] == t {
        return 1;
    }

    0
}

pub fn count_rays(input: &Vec<Vec<SeatType>>, index: (i64, i64), columns: i64, rows: i64) -> i64 {
    let mut rays = vec![];
    rays.push(ray((0, 1), input, index, columns, rows));
    rays.push(ray((0, -1), input, index, columns, rows));
    rays.push(ray((1, 0), input, index, columns, rows));
    rays.push(ray((-1, 0), input, index, columns, rows));
    rays.push(ray((1, 1), input, index, columns, rows));
    rays.push(ray((1, -1), input, index, columns, rows));
    rays.push(ray((-1, 1), input, index, columns, rows));
    rays.push(ray((-1, -1), input, index, columns, rows));
    return rays.iter().sum();
}

pub fn ray(
    direction: (i64, i64),
    input: &Vec<Vec<SeatType>>,
    index: (i64, i64),
    columns: i64,
    rows: i64,
) -> i64 {
    let mut current_index = (index.0 + direction.0, index.1 + direction.1);
    let mut tries = input.len() * input[0].len();
    let mut i = 1;
    while tries > 0 {
        if is_of_type(SeatType::O, input, current_index, columns, rows) == 1 {
            return 1;
        } else if is_of_type(SeatType::E, input, current_index, columns, rows) == 1 {
            return 0;
        }

        current_index = (index.0 + direction.0 * i, index.1 + direction.1 * i);

        tries -= 1;
        i += 1;
    }

    return 0;
}
