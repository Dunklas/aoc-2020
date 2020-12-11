use std::{collections::HashMap};

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let joltage_ratings = input_to_joltage_ratings(input);
    find_joltage_rating(joltage_ratings)
}

fn part_2(input: &str) -> u64 {
    let jolts = input_to_joltage_ratings(input);
    let target = jolts.iter().max().unwrap();
    let mut counts: HashMap<u64, u64> = HashMap::new();
    num_arrangements(0, *target, &jolts, &mut counts)
}

fn input_to_joltage_ratings(input: &str) -> Vec<u64> {
    let mut joltage_ratings: Vec<u64> = input.lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    joltage_ratings.sort();
    joltage_ratings.insert(0,0);
    let max = joltage_ratings.iter().max().unwrap();
    joltage_ratings.push(max + 3);
    joltage_ratings
}

fn find_joltage_rating(joltage_ratings: Vec<u64>) -> u64 {
    let mut diffs: HashMap::<u64,u64> = HashMap::new();
    for i in 1..joltage_ratings.len() {
        *diffs.entry(joltage_ratings[i] - joltage_ratings[i-1]).or_insert(0) += 1;
    }
    return diffs.get(&1).unwrap() * diffs.get(&3).unwrap();
}

fn num_arrangements(current: u64, target: u64, jolts: &Vec<u64>, counts: &mut HashMap<u64, u64>) -> u64{
    if current == target {
        return 1;
    }
    if counts.contains_key(&current) {
        return *counts.get(&current).unwrap();
    }
    let mut res = 0u64;
    for i in 1..4 {
        match jolts.binary_search(&(current + i)) {
            Ok(index) => {
                res += num_arrangements(*jolts.get(index).unwrap(), target, jolts, counts);
            },
            Err(_e) => {}
        }
    }
    counts.insert(current, res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        assert_eq!(part_1(&input), 7 * 5);
    }

    #[test]
    fn test_part_2_short() {
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        assert_eq!(part_2(&input), 8);
    }

    #[test]
    fn test_part_2_long() {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        assert_eq!(part_2(&input), 19208);
    }
}