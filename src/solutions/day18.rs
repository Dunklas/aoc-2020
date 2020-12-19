use regex::Regex;
use lazy_static::lazy_static;

enum Mode {
    Basic,
    Advanced
}

lazy_static! {
    static ref OP_AND_OPERAND: Regex = Regex::new(r"([\+|\*]?) (\d+)").unwrap();
    static ref DIGIT: Regex = Regex::new(r"(\d+)").unwrap();
    static ref ADDITION: Regex = Regex::new(r"\d+ \+ \d+").unwrap();
}

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i64 {
    input.lines()
        .map(|line| calculate(line, &Mode::Basic))
        .sum()
}

fn part_2(input: &str) -> i64 {
    input.lines()
        .map(|line| calculate(line, &Mode::Advanced))
        .sum()
}

fn calculate(row: &str, mode: &Mode) -> i64 {
    let mut copy = String::from(row);
    while let Some(i) = copy.find('(') {
        let mut open_count = 0;
        for j in i..copy.len() {
            match &copy[j..j+1] {
                "(" => open_count += 1,
                ")" => open_count -= 1,
                _ => {}
            };
            if open_count == 0 {
                copy.replace_range(i..j+1, &calculate(&copy[i+1..j], mode).to_string());
                break;
            }
        }
    }
    match mode {
        Mode::Basic => basic(&copy),
        Mode::Advanced => advanced(&copy),
    }
}

fn basic(row: &str) -> i64 {
    let mut total = match DIGIT.captures(row) {
        Some(cap) => cap[1].parse::<i64>().unwrap(),
        None => panic!("No digit found")
    };
    for cap in OP_AND_OPERAND.captures_iter(row) {
        let operator = &cap[1];
        let operand = &cap[2].parse::<i64>().unwrap();
        match operator {
            "+" => total += operand,
            "*" => total *= operand, 
            _ => {
                panic!("Unknown operator: {}", operator)
            }
        }
    }
    total 
}

fn advanced(row: &str) -> i64 {
    let mut copy = String::from(row);
    while let Some(mat) = ADDITION.find(&copy.clone()) {
        let x = &copy[mat.start()..mat.end()].split("+").collect::<Vec<&str>>();
        let result = x[0].trim().parse::<i64>().unwrap() + x[1].trim().parse::<i64>().unwrap();
        copy.replace_range(mat.start()..mat.end(), &result.to_string());
    }
    basic(&copy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_no_parenthesis() {
        assert_eq!(basic("1 + 2 * 3 + 4 * 5 + 6"), 71);
    }

    #[test]
    fn part_1_test_parenthesis() {
        assert_eq!(part_1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part_1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part_1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part_1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(part_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }

    #[test]
    fn part_2_test_no_parenthesis() {
        assert_eq!(advanced("1 + 2 * 3 + 4 * 5 + 6"), 231);
    }

    #[test]
    fn part_2_test_parenthesis() {
        assert_eq!(part_2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part_2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(part_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(part_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }
}
