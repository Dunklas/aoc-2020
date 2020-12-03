use std::{fmt, str, cmp};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(fmt::Debug, cmp::PartialEq, cmp::Eq)]
struct PasswordAndPolicy {
    password: String,
    lower: usize,
    upper: usize,
    character: char,
}

#[derive(fmt::Debug)]
struct ParsePasswordAndPolicyError;

impl str::FromStr for PasswordAndPolicy {
    type Err = ParsePasswordAndPolicyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<lower>\d{1,2})-(?P<upper>\d{1,2}) (?P<char>\w{1}): (?P<password>\w+)$").unwrap();
        }
        match RE.captures(&s) {
            Some(result) => {
                Ok(PasswordAndPolicy{
                    password: String::from(result.name("password")
                        .map(|pw| pw.as_str())
                        .ok_or(ParsePasswordAndPolicyError)?),
                    lower: result.name("lower")
                        .and_then(|lower| lower.as_str().parse().ok())
                        .ok_or(ParsePasswordAndPolicyError)?,
                    upper: result.name("upper")
                        .and_then(|upper| upper.as_str().parse().ok())
                        .ok_or(ParsePasswordAndPolicyError)?,
                    character: result.name("char")
                        .and_then(|c| c.as_str().parse().ok())
                        .ok_or(ParsePasswordAndPolicyError)?,
                })
            },
            None => Err(ParsePasswordAndPolicyError)
        }
    }
}

pub fn run(input: String) {
    let policies_and_passwords = to_policies_and_passwords(input);
    println!("Part 1: {}", check_passwords_part1(&policies_and_passwords));
    println!("Part 2: {}", check_passwords_part2(&policies_and_passwords));
}

fn to_policies_and_passwords(input: String) -> Vec<PasswordAndPolicy> {
    input.lines()
        .map(|line| line.parse::<PasswordAndPolicy>())
        .filter_map(Result::ok)
        .collect()
}

fn check_passwords_part1(password_and_policy_list: &Vec<PasswordAndPolicy>) -> u64 {
    let mut count = 0;
    for pp in password_and_policy_list {
        let c_count = pp.password.matches(pp.character).count();
        if c_count >= pp.lower && c_count <= pp.upper {
            count += 1;
        }
    }
    return count;
}

fn check_passwords_part2(password_and_policy_list: &Vec<PasswordAndPolicy>) -> u64 {
    let mut count = 0;
    for pp in password_and_policy_list {
        let first = pp.password.chars().nth(pp.lower - 1).unwrap();
        let second = pp.password.chars().nth(pp.upper - 1).unwrap();
        if (first == pp.character) ^ (second == pp.character) {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_line_to_password_and_policy() {
        let result: PasswordAndPolicy = "1-3 a: abcde".parse().unwrap();
        assert_eq!(result, PasswordAndPolicy{
            password: String::from("abcde"),
            lower: 1,
            upper: 3,
            character: 'a'
        });
    }

    #[test]
    fn part_1_test() {
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        let result = check_passwords_part1(&to_policies_and_passwords(input));
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_test() {
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        let result = check_passwords_part2(&to_policies_and_passwords(input));
        assert_eq!(result, 1);
    }
}
