use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MASK_PATTERN: Regex = Regex::new(r"^mask = ([\d|X]{36})").unwrap();
    static ref MEM_PATTERN: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
}

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let mut memory: HashMap::<usize, u64> = HashMap::new(); 
    let mut mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    input.lines()
        .for_each(|line| {
            match MASK_PATTERN.captures(line) {
                Some(cap) => {
                    mask = String::from(&cap[1]);
                },
                None => {},
            };
            match MEM_PATTERN.captures(line) {
                Some(cap) => {
                    let mem_index = &cap[1].parse::<usize>().unwrap();
                    let value = &cap[2].parse::<u64>().unwrap(); 
                    memory.insert(*mem_index, apply_mask_v1(&mask, *value));
                },
                None => {}
            };
        });
    memory.values().sum()
}

fn part_2(input: &str) -> u64 {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask = String::from("00000000000000000000000000000000000");
    input.lines()
        .for_each(|line| {
            match MASK_PATTERN.captures(line) {
                Some(cap) => {
                    mask = String::from(&cap[1]);
                },
                None => {},
            };
            match MEM_PATTERN.captures(line) {
                Some(cap) => {
                    let mem_index = &cap[1].parse::<usize>().unwrap();
                    let value = &cap[2].parse::<u64>().unwrap(); 
                    for address in apply_mask_v2(&mask, *mem_index) {
                        memory.insert(address, *value);
                    }
                },
                None => {}
            };
        });
    memory.values().sum()
}

fn apply_mask_v1(mask: &str, value: u64) -> u64 {
    let or_mask = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
    let and_mask = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
    value & and_mask | or_mask
}

fn apply_mask_v2(mask: &str, address: usize) -> Vec<usize> {
    let base_new = address | usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
    let mut floating_indices: Vec<usize> = Vec::new();
    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            'X' => floating_indices.push(i), 
            _ => {}
        }
    }
    let mut addresses: Vec<usize> = Vec::new();
    permutate(&floating_indices, 0, base_new, &mut addresses);
    addresses
}

fn permutate(indices: &Vec<usize>, i: usize, address: usize, result: &mut Vec<usize>) {
    if i == indices.len() as usize {
        result.push(address);
        return;
    }
    let mask = !(1usize << indices.get(i).unwrap());
    permutate(indices, i+1, address & mask, result);
    let mask = 1usize << indices.get(i).unwrap();
    permutate(indices, i+1, address | mask, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        assert_eq!(part_1(input), 165);
    }

    #[test]
    fn apply_mask_v1_test() {
        assert_eq!(apply_mask_v1("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11), 73);
        assert_eq!(apply_mask_v1("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 101), 101);
        assert_eq!(apply_mask_v1("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 0), 64);
    }

    #[test]
    fn apply_mask_v2_test() {
        let mut res = apply_mask_v2("000000000000000000000000000000X1001X", 42);
        res.sort();
        assert_eq!(res, vec![26, 27, 58, 59]);
        let mut res = apply_mask_v2("00000000000000000000000000000000X0XX", 26);
        res.sort();
        assert_eq!(res, vec![16, 17, 18, 19, 24, 25, 26, 27]);
    }
}
