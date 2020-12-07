use std::{str, collections};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENT_REGEX: Regex = Regex::new(r"(\d+) ([\w|\s]+) [bags|bag],*").unwrap();
}

pub fn parse_input(input: &str) -> collections::HashMap<String, Vec<String>> {
    let mut bag_rules = collections::HashMap::<String, Vec<String>>::new(); 
    for line in input.lines() {
        let parts: Vec<&str> = line.split("contain").collect();
        let container_color = parts[0].replace(" bags ", "");
        for cap in CONTENT_REGEX.captures_iter(parts[1]) {
            let count = &cap[1].parse::<u16>().unwrap();
            let color = &cap[2];
            match bag_rules.get_mut(color) {
                Some(containers) => {
                    containers.push(String::from(container_color.clone()));
                },
                None => {
                    let containers = vec![container_color.clone()]; 
                    bag_rules.insert(String::from(color), containers);
                }
            }
        }
    }
    bag_rules
}

pub fn run(input: &str) {
    println!("{}", part_1(input));
}

fn part_1(input: &str) -> u64 {
    let something = parse_input(input);
    let shiny_gold = something.get("shiny gold").unwrap();
    let mut containers = 0u64;
    println!("{:?}", shiny_gold);
    return 0;
}

fn number_of_containers() -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {

    }

    #[test]
    fn test_part_2() {

    }
}
