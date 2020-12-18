use std::{ops::Range, str::FromStr, fmt::Debug, collections::HashSet, clone::Clone, hash::Hash, cmp::{PartialEq, Eq}};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref FIELD_RULE_PATTERN: Regex = Regex::new(r"(\w+\s?\w*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct FieldRule {
    name: String,
    ranges: (Range<u64>, Range<u64>),
}

impl FieldRule {
    fn valid_for(self: &Self, value: u64) -> bool {
        self.ranges.0.contains(&value) || self.ranges.1.contains(&value)
    }
}

struct ParseFieldRuleError;
impl FromStr for FieldRule {
    type Err = ParseFieldRuleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match FIELD_RULE_PATTERN.captures(s) {
            Some(cap) => Ok(FieldRule {
                name: String::from(&cap[1]),
                ranges: (
                    Range{ start: cap[2].parse().unwrap(), end: cap[3].parse::<u64>().unwrap() + 1},
                    Range{ start: cap[4].parse().unwrap(), end: cap[5].parse::<u64>().unwrap() + 1}
                ) 
            }),
            None => Err(ParseFieldRuleError)
        }
    }
}

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let (field_rules, _ticket, nearby_tickets) = parse_input(input);
    nearby_tickets.iter()
        .map(|ticket| ticket.iter().fold(0, |acc, v|
            match !field_rules.iter().any(|rule| rule.valid_for(*v)) {
                true => *v, 
                false => acc 
            })
        ).sum::<u64>()
}

fn part_2(input: &str) -> u64 {
    let (field_rules, ticket, nearby_tickets) = parse_input(input);
    let valid_tickets: Vec<Vec<u64>> = nearby_tickets.into_iter()
        .filter(|ticket| ticket.iter().all(|v|
            field_rules.iter().any(|rule| rule.valid_for(*v))
        )).collect();
    let mut result = 1u64;
    let mut remaining_rules: HashSet::<FieldRule> = field_rules.to_vec().into_iter()
        .collect();
    loop {
        if remaining_rules.is_empty() {
            break;
        }
        let indices: Vec<usize> = field_rules.clone().into_iter().enumerate()
            .map(|(i, _ticket)| i)
            .collect();
        indices.iter().for_each(|i| {
            let candidates: Vec<&FieldRule> = field_rules.iter()
                .filter(|rule| remaining_rules.contains(rule))
                .filter(|rule| valid_tickets.iter()
                    .map(|ticket| *ticket.get(*i).unwrap())
                    .all(|v| rule.valid_for(v))
                ).collect();
            if candidates.len() == 1 {
                remaining_rules.remove(candidates.first().unwrap());
                if candidates.first().unwrap().name.contains("departure") {
                    result *= *ticket.get(*i).unwrap();
                }
            }
        });
    }
    result
}

fn parse_input(input: &str) -> (Vec<FieldRule>, Vec<u64>, Vec<Vec<u64>>){
    let groups: Vec<&str> = input.split("\n\n").collect();
    let field_rules: Vec<FieldRule> = groups[0].lines()
        .filter_map(|line| line.parse::<FieldRule>().ok())
        .collect(); 
    let tmp: Vec<&str> = groups[1].lines().collect();
    let ticket: Vec<u64> = tmp[1].split(",").filter_map(|v| v.parse().ok()).collect();
    let nearby_tickets: Vec<Vec<u64>> = groups[2].lines()
        .filter(|line| !line.contains("nearby tickets:"))
        .map(|line| line.split(",").filter_map(|v| v.parse().ok()).collect())
        .collect();
    (field_rules, ticket, nearby_tickets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";
        assert_eq!(part_1(input), 71);
    }

    #[test]
    fn part_2_test() {
        let input = "departure class: 0-1 or 4-19\ndeparture row: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9\n20,21,22";
        assert_eq!(part_2(input), 132);
    }
}