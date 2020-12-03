use std::{fmt, cmp};
use crate::grid::grid;

#[derive(fmt::Debug, cmp::PartialEq, cmp::Eq)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position{
            x, y
        }
    }
}

pub fn run(input: String) {
    let forest_grid: grid::Grid<char> = input.parse().unwrap();
    println!("Part 1: {}", find_tree_obstacles(&forest_grid, 3, 1));

    let mut accumulator = 1;
    for (move_x, move_y) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        accumulator *= find_tree_obstacles(&forest_grid, move_x, move_y);
    }
    println!("Part 2: {}", accumulator);
}

fn find_tree_obstacles(grid: &grid::Grid<char>, move_x: usize, move_y: usize) -> u32 {
    let mut current_pos = Position::new(0, 0);
    let mut obstacle_count = 0;
    loop {
        current_pos = Position::new(current_pos.x + move_x, current_pos.y + move_y);
        let translated_pos = translate_position(grid.width().unwrap(), &current_pos);
        if translated_pos.y >= grid.height() {
            break;
        }
        let object_on_grid = grid.at(translated_pos.x, translated_pos.y).unwrap();
        if object_on_grid == '#' {
            obstacle_count += 1;
        }
    }
    return obstacle_count; 
}

fn translate_position(width: usize, pos: &Position) -> Position {
    Position::new(pos.x % width, pos.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_test_input() -> String {
        fs::read_to_string("src/solutions/day3.test-input")
            .expect("No test-input file for day 3 found")
    }

    #[test]
    fn should_find_right_number_of_tree_obstacles() {
        let grid: grid::Grid<char> = read_test_input().parse().unwrap();
        assert_eq!(find_tree_obstacles(&grid, 3, 1), 7);
    }

    #[test]
    fn should_translate_pos() {
        let translated_pos = translate_position(11, &Position::new(11, 0));
        assert_eq!(translated_pos, Position::new(0, 0));
    }
}
