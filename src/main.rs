use std::{env, fs};

mod grid;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].as_str();
    let input = read_puzzle_input(day);
    match args[1].as_str() {
        "1" => solutions::day1::run(input),
        "2" => solutions::day2::run(input),
        "3" => solutions::day3::run(input),
        "4" => solutions::day4::run(input),
        "5" => solutions::day5::run(input),
        "6" => solutions::day6::run(input),
        "7" => solutions::day7::run(&input),
        "8" => solutions::day8::run(&input),
        "9" => solutions::day9::run(&input),
        "10" => solutions::day10::run(&input),
        "11" => solutions::day11::run(&input),
        "12" => solutions::day12::run(&input),
        "13" => solutions::day13::run(&input),
        "14" => solutions::day14::run(&input),
        "15" => solutions::day15::run(&input),
        "16" => solutions::day16::run(&input),
        "17" => solutions::day17::run(&input),
        _ => println!("Day {} not implemented", args[1])
    };
}

fn read_puzzle_input(day: &str) -> String {
    fs::read_to_string(format!("src/solutions/day{}.input", day))
        .expect(&format!("No input file for day {} found", day))
}
