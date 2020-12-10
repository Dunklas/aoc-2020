use std::{cmp};

pub fn run(input: &str) {
    let numbers: Vec<u64> = input.lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    let invalid = part_1(&numbers, 25);
    println!("Part 1: {}", invalid);
    println!("Part 2: {}", part_2(invalid, &numbers));
}

fn part_1(numbers: &Vec<u64>, preamble_size: usize) -> u64 {
    for i in preamble_size..numbers.len() {
        let n = numbers[i];
        let preamble = &numbers[i-preamble_size..i];
        if !sum_of_preamble(n, preamble.to_vec()) {
            return n;
        }
    }
    0
}

fn part_2(invalid_number: u64, all_numbers: &Vec<u64>) -> u64 {
    for i in 0..all_numbers.len() {
        let mut sum = all_numbers[i];
        for j in i+1..all_numbers.len() {
            if sum == invalid_number {
                let mut range = all_numbers[i..j].to_vec();
                range.sort();
                return range[0] + range[range.len()-1];
            }
            sum += all_numbers[j];
        }
    }
    0
}

fn sum_of_preamble(n: u64, mut preamble: Vec<u64>) -> bool {
    preamble.sort();
    let mut left = 0;
    let mut right = preamble.len()-1;
    while left < right {
        let sum = preamble[left] + preamble[right];
        match sum.cmp(&n) {
            cmp::Ordering::Less => {left += 1},
            cmp::Ordering::Greater => {right -= 1},
            cmp::Ordering::Equal => return true,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_numbers(input: &str) -> Vec<u64> {
        input.lines()
            .filter_map(|line| line.parse().ok())
            .collect()
    }

    #[test]
    fn part_1_test() {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        assert_eq!(part_1(&to_numbers(&input), 5), 127);
    }

    #[test]
    fn part_2_test() {
        let numbers = to_numbers("35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576");
        let invalid_number = part_1(&numbers, 5);
        assert_eq!(part_2(invalid_number, &numbers), 62);
    }
}
