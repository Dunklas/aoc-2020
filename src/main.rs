use std::{env, fs};

mod grid;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].as_str();
    let input = read_puzzle_input(day);
    match args[1].as_str() {
        "1" => {
            solutions::day1::run(input);
        },
        "2" => {
            solutions::day2::run(input);
        },
        "3" => {
            solutions::day3::run(input);
        },
        "4" => {
            solutions::day4::run(input);
        },
        "5" => {
            solutions::day5::run(input);
        },
        "6" => {
            solutions::day6::run(input);
        },
        "7" => {
            solutions::day7::run(&input);
        }
        _ => {
            println!("Day {} not implemented", args[1]);
        }
    }
}

fn read_puzzle_input(day: &str) -> String {
    fs::read_to_string(format!("src/solutions/day{}.input", day))
        .expect(&format!("No input file for day {} found", day))
}
