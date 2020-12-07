use std::{str, collections::HashMap};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONTENT_REGEX: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
}

type BagRules = HashMap<String, HashMap<String, u16>>;
pub fn parse_input(input: &str) -> BagRules {
    let mut bag_rules = BagRules::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("contain").collect();
        let container_color = parts[0].replace(" bags ", "");
        let mut content = HashMap::<String, u16>::new();
        for cap in CONTENT_REGEX.captures_iter(parts[1]) {
            content.insert(
                String::from(&cap[2]),
                cap[1].parse().unwrap()
            );
        }
        bag_rules.insert(container_color, content);
    }
    bag_rules
}

pub fn run(input: &str) {
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let bag_rules = parse_input(input);
    bag_rules.keys()
        .filter(|color| contains_shiny_gold(color, &bag_rules))
        .count() as u64
}

fn part_2(input: &str) -> u64 {
    let bag_rules = parse_input(input);
    return number_of_bags("shiny gold", &bag_rules);
}

fn contains_shiny_gold(color: &str, bag_rules: &BagRules) -> bool {
    let bag = bag_rules.get(color).unwrap();
    match bag.contains_key("shiny gold") {
        true => true,
        false => bag.keys()
            .any(|c| contains_shiny_gold(c, &bag_rules))
    }
}

fn number_of_bags(color: &str, bag_rules: &BagRules) -> u64 {
    let contents = bag_rules.get(color).unwrap();
    let mut total = 0u64;
    for (color, count) in contents {
        total += *count as u64;
        total += number_of_bags(color, bag_rules) * *count as u64;
    }
    total 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags."
        ].join("\n");
        assert_eq!(part_1(&input), 4);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags."
        ].join("\n");
        assert_eq!(part_2(&input), 32);
    }
}
