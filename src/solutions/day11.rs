use crate::grid::grid::Grid;
use crate::grid::coordinate::CartesianCoordinate;

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let mut grid: Grid<char> = input.parse().unwrap();
    let coordinates = grid.coordinates();
    loop {
        let mut updates: Vec<(CartesianCoordinate, char)> = Vec::new();
        coordinates.iter()
            .map(|coord| (coord, grid.at(coord).unwrap()))
            .filter(|c| c.1 == '#' || c.1 == 'L')
            .for_each(|(coord, element)| {
                let adjacent_occupied = grid.adjacent_to(&coord, '#');
                if adjacent_occupied == 0 && element == 'L' {
                    updates.push((*coord, '#'))
                }
                if adjacent_occupied >= 4 && element == '#' {
                    updates.push((*coord, 'L'));
                }
            });
        if updates.is_empty() {
            break;
        }
        for update in updates {
            grid.set(&update.0, update.1).unwrap();
        }
    }
    grid.number_of('#')
}

fn part_2(input: &str) -> usize {
    let mut grid: Grid<char> = input.parse().unwrap();
    let coordinates = grid.coordinates();
    loop {
        let mut updates: Vec<(CartesianCoordinate, char)> = Vec::new();
        coordinates.iter()
            .map(|coord| (coord, grid.at(coord).unwrap()))
            .filter(|c| c.1 == '#' || c.1 == 'L') 
            .for_each(|(coord, element)| {
                let visible_occupied = number_visible_occupied_seats(&grid, coord);
                if visible_occupied == 0 && element == 'L' {
                    updates.push((*coord, '#'));
                }
                if visible_occupied >= 5 && element == '#' {
                    updates.push((*coord, 'L'));
                }
            });
        if updates.is_empty() {
            break;
        }
        for update in updates {
            grid.set(&update.0, update.1).unwrap();
        }
    }
    grid.number_of('#')
}

fn number_visible_occupied_seats(grid: &Grid<char>, from: &CartesianCoordinate) -> usize {
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let width = grid.width().unwrap() as isize;
    let height = grid.height() as isize;
    let mut num_occupied = 0;
    for (x_step, y_step) in directions {
        let mut x = from.x as isize;
        let mut y = from.y as isize;
        loop {
            x += x_step;
            y += y_step;
            if (x < 0 || x >= width) || (y < 0 || y >= height) {
                break;
            }
            match grid.at(&CartesianCoordinate::new(x as usize, y as usize)).unwrap() {
                '#' => {
                    num_occupied += 1;
                    break;
                },
                'L' => {
                    break;
                }
                _ => {}
            }
        }
    }
    num_occupied
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_test_input() -> String {
        fs::read_to_string("src/solutions/day11.test-input")
            .expect("No test-input file for day 3 found")
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&read_test_input()), 37);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&read_test_input()), 26);
    }

    #[test]
    fn test_number_visible_occupied_seats() {
        let grid = ".......#.\n...#.....\n.#.......\n.........\n..#L....#\n....#....\n.........\n#........\n...#.....".parse::<Grid<char>>().unwrap();
        assert_eq!(number_visible_occupied_seats(&grid, &CartesianCoordinate::new(3,4)), 8);
    }

    #[test]
    fn test_number_visible_occupied_seats2() {
        let grid = ".............\n.L.L.#.#.#.#.\n.............".parse::<Grid<char>>().unwrap();
        assert_eq!(number_visible_occupied_seats(&grid, &CartesianCoordinate::new(1,1)), 0);
    }

    #[test]
    fn test_number_visible_occupied_seats3() {
        let grid = ".##.##.\n#.#.#.#\n##...##\n...L...\n##...##\n#.#.#.#\n.##.##.".parse::<Grid<char>>().unwrap();
        assert_eq!(number_visible_occupied_seats(&grid, &CartesianCoordinate::new(3,3)), 0);
    }
}
