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
            let resp = client.get(&format!(
                "https://adventofcode.com/{}/day/{}/input",
                year, day
            )).header("Cookie", include_str!("./config.bin"));
            let body = resp.send().await?.text().await?;

            fs::write(&path, body.as_bytes())?;

            body
        }
    };

    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(solutions::problem_1::Problem1::new()),
        Box::new(solutions::problem_2::Problem2::new()),
    ];

    let mut i = 1;
    for mut s in solutions {
        println!("--------------------------------------");
        let el = std::time::Instant::now();
        s.as_mut().run(&data);
        let elapsed = el.elapsed();
        println!("Execution time {:?}, problem {}", elapsed, i);
        i += 1;
    }
    println!("--------------------------------------");

    Ok(())
}

trait Solution {
    fn run(&mut self, _: &String) {}
    fn test(&mut self) {}
}