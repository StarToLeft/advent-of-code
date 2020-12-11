use chrono::prelude::*;

use std::env;
use std::fs;

#[macro_use]
extern crate colour;

#[path = "./solutions/solutions.rs"]
pub mod solutions;

pub mod helper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_date = Local::now();
    let mut day = current_date.day();
    let month = current_date.month();
    let mut year = current_date.year();

    let arguments: Vec<_> = env::args().collect();
    if arguments.len() > 1 {
        year = arguments[1].parse::<i32>()?;
    }

    if arguments.len() > 2 {
        day = arguments[2].parse::<u32>()?;
    }

    let path = format!("./data/{}-{}-{}.bin", day, month, year);
    let res = fs::read_to_string(&path);
    let data = match res {
        Ok(data) => data,
        Err(_) => {
            println!("No local cache found, fetching input.");

            let client = reqwest::Client::new();
            let resp = client
                .get(&format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    year, day
                ))
                .header("Cookie", include_str!("./config.bin"));
            let body = resp.send().await?.text().await?;

            fs::write(&path, body.as_bytes())?;

            body
        }
    };

    let mut solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(solutions::problem_1::Problem1::new()),
        Box::new(solutions::problem_2::Problem2::new()),
    ];

    #[cfg(feature = "benchmark")]
    {
        red_ln!("Benchmarking...");
        let mut times_problem_1 = vec![];
        let mut times_problem_2 = vec![];
        let tries = 10;
        for _ in 0..tries {
            for (i, s) in solutions.iter_mut().enumerate() {
                let el = std::time::Instant::now();
                s.as_mut().run(&data);
                let elapsed = el.elapsed();

                if i == 0 {
                    times_problem_1.push(elapsed.as_millis());
                } else {
                    times_problem_2.push(elapsed.as_millis());
                }
            }
        }

        let x: i128 = times_problem_1.iter().sum::<u128>() as i128 / times_problem_1.len() as i128;
        let y: i128 = times_problem_2.iter().sum::<u128>() as i128 / times_problem_1.len() as i128;

        red_ln!("Average time solution 1: {:?}ms", x);
        red_ln!("Average time solution 2: {:?}ms", y);
    }

    let mut i = 1;
    for mut s in solutions {
        red_ln!("--------------------------------------");
        let el = std::time::Instant::now();
        s.as_mut().run(&data);
        let elapsed = el.elapsed();
        yellow_ln!("Full execution time {:?}, problem {}", elapsed, i);
        i += 1;
    }
    red_ln!("--------------------------------------");

    Ok(())
}

trait Solution {
    fn run(&mut self, _: &String) {}
}
