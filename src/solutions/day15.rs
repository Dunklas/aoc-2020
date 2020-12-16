use std::cmp::Ordering;

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    play(input, 2020)
}

fn part_2(input: &str) -> u32 {
    play(input, 30000000)
}

fn play(input: &str, day: u32) -> u32 {
    let mut last: Vec<u32> = vec![0u32; 30000000];
    let start_numbers: Vec<u32> = input.split(",").filter_map(|n| n.parse().ok()).collect();
    for (i, n) in start_numbers.iter().enumerate() {
        last[*n as usize] = (i + 1) as u32;
    }
    let mut round = start_numbers.len() as u32;
    let mut prev = *start_numbers.last().unwrap();
    loop {
        let prev_round = last[prev as usize];
        match prev_round.cmp(&0) {
            Ordering::Equal => {
                last[prev as usize] = round;
                prev = 0;
            },
            _ => {
                let diff = round - prev_round;
                last[prev as usize] = round;
                prev = diff;
            }
        }
        round += 1;
        if round == day {
            break; 
        }
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("0,3,6"), 436);
    }
}