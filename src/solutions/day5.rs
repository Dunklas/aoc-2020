pub fn run(input: String) {
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}

fn part_1(input: &String) -> usize {
    input.lines()
        .map(|line| seat_to_id(line))
        .max()
        .unwrap()
}

fn part_2(input: &String) -> usize {
    let mut passes: Vec<usize> = input.lines()
        .map(|line| seat_to_id(line))
        .collect();
    passes.sort();
    for i in 1..passes.len() {
        if passes.get(i).unwrap() - passes.get(i-1).unwrap() == 2 {
            return *passes.get(i).unwrap() - 1;
        }
    }
    return 0;
}

fn seat_to_id(seat: &str) -> usize {
    usize::from_str_radix(
        &seat.replace("R", "1").replace("B", "1").replace("L", "0").replace("F", "0"),
        2
    ).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_to_row_and_col_test() {
        assert_eq!(seat_to_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_to_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_to_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_to_id("BBFFBBFRLL"), 820);
    }
}