use std::fmt::Debug;
use Rule::*;

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAR_RULE_PATTERN: Regex = Regex::new(r#""(\w{1})""#).unwrap();
    static ref AND_RULE_PATTERN: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Rule {
    SingleCharacter(String),
    CompositeRule(Vec<Vec<usize>>),
}

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let (rules, messages) = parse(input);
    let rule_0 = Regex::new(&format!(r"^{}$", into_regex(&rules, rules.get(0).unwrap()))).unwrap();
    messages.iter()
        .filter(|message| rule_0.is_match(message))
        .count() as u64
}

fn part_2(input: &str) -> u64 {
    let (rules, messages) = parse(input);
    let rule_42 = Regex::new(&format!(r"({})", into_regex(&rules, rules.get(42).unwrap()))).unwrap();
    let rule_31 = Regex::new(&format!(r"({})", into_regex(&rules, rules.get(31).unwrap()))).unwrap();
    let chunk_size = 8;
    messages.into_iter()
        .filter(|message| {
            let chunks = message.chars().collect::<Vec<char>>();
            let chunks = chunks.chunks(chunk_size)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();
            let mut count_42 = 0;
            for i in 0..chunks.len() {
                match rule_42.is_match(&chunks[i]) {
                    true => count_42 += 1,
                    false => break
                };
            }
            let mut count_31 = 0;
            for i in count_42+count_31..chunks.len() {
                match rule_31.is_match(&chunks[i]) {
                    true => count_31 += 1,
                    false => break
                };
            }
            // count_42 + count_31: Must match complete string
            // count_31 - count_42 > -1: Since there must be at last one more match of rule 42, than for rule 31
            // count_31 == 0: Since rule 31 must be matched at least once
            if count_31 == 0 || (count_42 + count_31) != chunks.len() || (count_31 as isize) - (count_42 as isize) > -1 {
                return false;
            }
            true
        })
        .count() as u64
}

fn into_regex(rules: &Vec<Rule>, rule: &Rule) -> String {
    match rule {
        SingleCharacter(c) => c.to_string(),
        CompositeRule(sub_rules) => {
            let sub_rules = sub_rules.iter()
                .map(|sub_rule| sub_rule
                    .iter()
                    .map(|id| into_regex(rules, &rules[*id]))
                    .collect::<Vec<_>>()
                    .join("")
                )
                .collect::<Vec<_>>();
            if sub_rules.len() > 1 {
                format!("(?:{})", sub_rules.join("|"))
            } else {
                sub_rules.join("")
            }
        }
    }
}

fn parse(input: &str) -> (Vec<Rule>, Vec<&str>) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let num_rules = &parts[0].split("\n").collect::<Vec<&str>>().len();
    let mut completed_rules = vec![SingleCharacter("".to_owned()); *num_rules];
    parts[0].lines()
        .for_each(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let id = parts[0].parse::<usize>().unwrap();
            match CHAR_RULE_PATTERN.captures(parts[1]) {
                Some(cap) => {
                    let character = cap[1].parse::<char>().unwrap();
                    completed_rules[id] = SingleCharacter(character.to_string());
                },
                None => {
                    let sub_rule = parts[1].split(" | ")
                        .map(|seq| seq.split_ascii_whitespace()
                            .map(|sub_id| sub_id.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                        )
                        .collect::<Vec<Vec<usize>>>();
                    completed_rules[id] = CompositeRule(sub_rule);
                }
            };
        });
    let messages = parts[1].lines().collect::<Vec<&str>>();
    (completed_rules, messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb";
        assert_eq!(part_1(input), 2);
    }

}
