use std::{cmp};

pub fn run(input: String) {
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}

fn part_1(input: &String) -> usize {
    input.lines()
        .map(|line| seat_to_row_and_col(line))
        .map(|row_and_col| row_and_col.0 * 8 + row_and_col.1)
        .max()
        .unwrap()
}

fn part_2(input: &String) -> usize {
    let mut passes: Vec<usize> = input.lines()
        .map(|line| seat_to_row_and_col(line))
        .map(|row_and_col| row_and_col.0 * 8 + row_and_col.1)
        .collect();
    passes.sort();
    for i in 1..passes.len() {
        if passes.get(i).unwrap() - passes.get(i-1).unwrap() == 2 {
            return *passes.get(i).unwrap() - 1;
        }
    }
    return 0;
}

fn seat_to_row_and_col(seat: &str) -> (usize, usize) {
    let mut row: (usize, usize) = (0, 127);
    let mut col: (usize, usize) = (0, 7);
    for c in seat.chars() {
        let row_diff = cmp::max(row.0, row.1) - cmp::min(row.0, row.1) + 1;
        let col_diff = cmp::max(col.0, col.1) - cmp::min(col.0, col.1) + 1;
        match c {
            'F' => row = (row.0, row.1 - (row_diff / 2)),
            'B' => row = (row.0 + row_diff / 2, row.1),
            'L' => col = (col.0, col.1 - (col_diff / 2)), 
            'R' => col = (col.0 + col_diff / 2, col.1),
            _ => {}
        }
    }
    (row.0, col.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_to_row_and_col_test() {
        assert_eq!(seat_to_row_and_col("FBFBBFFRLR"), (44, 5));
        assert_eq!(seat_to_row_and_col("BFFFBBFRRR"), (70, 7));
        assert_eq!(seat_to_row_and_col("FFFBBBFRRR"), (14, 7));
        assert_eq!(seat_to_row_and_col("BBFFBBFRLL"), (102, 4));
    }
}