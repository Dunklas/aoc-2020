use std::{ops::RangeInclusive, collections::HashSet};

type Cube = Vec<isize>;

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    simulate(input, 2)
}

fn part_2(input: &str) -> u64 {
    simulate(input, 3)
}

fn simulate(input: &str, dimensions: usize) -> u64 {
    let mut space = parse(input, dimensions);
    let mut border = calculate_border(&space, dimensions); 
    for _i in 0..6 {
        let mut next_space: HashSet<Cube> = HashSet::new();
        apply_changes(&space, &mut next_space, &border, dimensions, 0, Vec::new());
        border = calculate_border(&next_space, dimensions);
        space = next_space;
    }
    space.len() as u64
}

fn calculate_border(space: &HashSet<Cube>, dimensions: usize) -> Vec<RangeInclusive<isize>> {
    let mut border: Vec<RangeInclusive<isize>> = Vec::new();
    for d in 0..dimensions+1 {
        let values: Cube = space.iter()
            .map(|coord| *coord.get(d).unwrap())
            .collect();
        border.push(RangeInclusive::new(
            values.iter().min().unwrap_or(&0) - 1,
            values.iter().max().unwrap_or(&0) + 1
        ))
    }
    border
}

fn parse(input: &str, dimensions: usize) -> HashSet<Cube> {
    let mut space: HashSet<Cube> = HashSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                '#' => {
                    let mut coordinate = vec![0isize; dimensions+1];
                    coordinate[dimensions-1] = y as isize;
                    coordinate[dimensions] = x as isize;
                    space.insert(coordinate);
                },
                _ => {}
            };
        });
    });
    space
}

fn apply_changes(space: &HashSet<Cube>, next_space: &mut HashSet<Cube>, border: &Vec<RangeInclusive<isize>>, target_dimension: usize, dimension: usize, coordinate: Cube) {
    if dimension > target_dimension {
        let active_neighbours = active_neighbours(&coordinate.clone(), space, target_dimension, 0, Vec::new());
        match space.get(&coordinate) {
            Some(_cube) => {
                if active_neighbours == 2 || active_neighbours == 3 {
                    next_space.insert(coordinate);
                }
            },
            None => {
                if active_neighbours == 3 {
                    next_space.insert(coordinate);
                }
            },
        }
        return;
    }
    for n in border.get(dimension).unwrap().clone() {
        let mut coordinate = coordinate.clone();
        coordinate.push(n);
        apply_changes(space, next_space, border, target_dimension, dimension + 1, coordinate);
    }
}

fn active_neighbours(source: &Cube, space: &HashSet<Cube>, target_dimension: usize, dimension: usize, coordinate: Cube) -> usize {
    if coordinate == *source {
        return 0;
    }
    if dimension > target_dimension {
        match space.contains(&coordinate) {
            true => return 1,
            false => return 0,
        }
    }
    let mut sum = 0usize;
    let s = source.get(dimension).unwrap();
    for n in s - 1..=s + 1 {
        let mut coordinate = coordinate.clone();
        coordinate.push(n);
        sum += active_neighbours(source, space, target_dimension, dimension + 1, coordinate);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = ".#.\n..#\n###";
        assert_eq!(part_1(input), 112);
    }

    #[test]
    fn part_2_test() {
        let input = ".#.\n..#\n###";
        assert_eq!(part_2(input), 848);
    }
}
