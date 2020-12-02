use std::fmt;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(fmt::Debug)]
struct PasswordPolicy {
    lower: usize,
    upper: usize,
    character: char,
}

pub fn run(input: String) {
    let policies_and_passwords = to_policies_and_passwords(input);
    println!("Part 1: {}", part_1(&policies_and_passwords));
    println!("Part 2: {}", part_2(&policies_and_passwords));
}

fn to_policies_and_passwords(input: String) -> Vec<(PasswordPolicy, String)> {
    input.split("\n")
        .into_iter()
        .map(|line| line_to_password_and_policy(line))
        .filter_map(|x| x)
        .collect()
}

fn line_to_password_and_policy(line: &str) -> Option<(PasswordPolicy, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<lower>\d{1,2})-(?P<upper>\d{1,2}) (?P<char>\w{1}): (?P<password>\w+)$").unwrap();
    }
    match RE.captures(&line) {
        Some(result) => {
            let lower: usize = result.name("lower")
                .and_then(|lower| lower.as_str().parse().ok())?;
            let upper: usize = result.name("upper")
                .and_then(|upper| upper.as_str().parse().ok())?;
            let character: char = result.name("char")
                .and_then(|c| c.as_str().parse().ok())?;
            let password: &str = result.name("password")
                .map(|pw| pw.as_str())?;
            return Some((PasswordPolicy{
                lower, 
                upper,
                character,
            }, String::from(password)));
        },
        None => None
    }
}

fn part_1(password_and_policy_list: &Vec<(PasswordPolicy, String)>) -> u64 {
    let mut count = 0;
    for (policy, password) in password_and_policy_list {
        let c_count = password.matches(policy.character).count();
        if c_count >= policy.lower && c_count <= policy.upper {
            count += 1;
        }
    }
    return count;
}

fn part_2(password_and_policy_list: &Vec<(PasswordPolicy, String)>) -> u64 {
    let mut count = 0;
    for (policy, password) in password_and_policy_list {
        let first = password.chars().nth(policy.lower - 1).unwrap();
        let second = password.chars().nth(policy.upper - 1).unwrap();
        if (first == policy.character && second != policy.character) || (first != policy.character && second == policy.character) {
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
        let line = "1-3 a: abcde";
        let result = line_to_password_and_policy(line);
        assert_eq!(result.is_some(), true);
        let result = result.unwrap();
        assert_eq!(result.0.lower, 1);
        assert_eq!(result.0.upper, 3);
        assert_eq!(result.0.character, 'a');
        assert_eq!(result.1, "abcde".to_string());
    }

    #[test]
    fn part_1_test() {
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        let result = part_1(&to_policies_and_passwords(input));
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_test() {
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        let result = part_2(&to_policies_and_passwords(input));
        assert_eq!(result, 1);
    }
}
