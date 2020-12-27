use std::str::{FromStr};
use std::fmt;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
struct Tile {
    id: usize,
    grid: Vec<Vec<char>>
}
struct ParseTileError;
impl FromStr for Tile {
    type Err = ParseTileError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines.next().unwrap()[5..9].parse::<usize>().unwrap();
        let mut grid = vec![vec!['\0'; 10]; 10];
        lines.enumerate().for_each(|(i, row)| row.chars().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
        }));
        Ok(Self{
            id,
            grid,
        })
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tmp = String::new();
        for i in 0..10 {
            for j in 0..10 {
                tmp.push(self.grid[i][j]);
            }
            tmp.push('\n');
        }
        write!(f, "{}", tmp)
    }
}
impl Tile {
    fn rotate(&mut self) {
        rotate(&mut self.grid);
    }
    fn flip_horizontal(&mut self) {
        flip_horizontal(&mut self.grid);
    }
    fn flip_vertical(&mut self) {
        flip_vertical(&mut self.grid);
    }
    fn upper_edge(&self) -> String {
        self.grid[0].iter().collect()
    }
    fn lower_edge(&self) -> String {
        self.grid[9].iter().collect()
    }
    fn left_edge(&self) -> String {
        self.grid.iter()
            .map(|row| row.iter().nth(0).unwrap())
            .collect()
    }
    fn right_edge(&self) -> String {
        self.grid.iter()
            .map(|row| row.iter().nth(9).unwrap())
            .collect()
    }
    fn neighbour(&self, tiles_by_edge: &HashMap<String, Vec<usize>>, direction: Direction) -> Option<usize> {
        let neighbour_ids = match direction {
            Direction::Up => tiles_by_edge.get(&self.upper_edge()),
            Direction::Down => tiles_by_edge.get(&self.lower_edge()),
            Direction::Left => tiles_by_edge.get(&self.left_edge()),
            Direction::Right => tiles_by_edge.get(&self.right_edge())
        };
        match neighbour_ids {
            Some(ids) => ids.iter()
                .find(|&&id| id != self.id)
                .copied(),
            None => None
        }
    }
}

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let tiles: HashMap<usize, Tile> = input.split("\n\n")
        .filter_map(|text| text.parse::<Tile>().ok())
        .map(|tile| (tile.id, tile))
        .collect();
    let tiles_by_edge = tiles_by_edge(&tiles);
    find_corners(&tiles_by_edge)
        .iter()
        .product::<usize>() as u64
}

fn part_2(input: &str) -> u64 {
    let tiles: HashMap<usize, Tile> = input.split("\n\n")
        .filter_map(|text| text.parse::<Tile>().ok())
        .map(|tile| (tile.id, tile))
        .collect();
    let tiles_by_edge = tiles_by_edge(&tiles);
    let corners = find_corners(&tiles_by_edge);
    let final_image = construct_image(&tiles_by_edge, &tiles, *corners.first().unwrap());
    let total_squares = final_image.iter()
        .flatten()
        .filter(|&&c| c == '#')
        .count() as u64;
    let manipulations: Vec<Box<dyn Fn(&mut Vec<Vec<char>>)>> = vec![
        Box::new(|_image| {}),
        Box::new(|image| { flip_vertical(image) }),
        Box::new(|image| { flip_horizontal(image) }),
        Box::new(|image| {
            flip_vertical(image);
            flip_horizontal(image);
        })
    ];
    for manipulation in manipulations {
        let mut temp = final_image.clone();
        manipulation(&mut temp);
        for _i in 0..3 {
            match num_sea_monsters(&temp) {
                0 => rotate(&mut temp),
                num_monsters => return total_squares - num_monsters
            }
        }
    }
    0
}

fn tiles_by_edge(tiles: &HashMap<usize, Tile>) -> HashMap<String, Vec<usize>> {
    let mut tiles_by_edge: HashMap<String, Vec<usize>> = HashMap::new();
    tiles.values()
        .for_each(|tile| {
            let left = tile.left_edge();
            let right = tile.right_edge();
            let up = tile.upper_edge();
            let down = tile.lower_edge();
            for edge in &[up, down, left, right] {
                tiles_by_edge.entry(edge.to_string()).or_insert(Vec::new()).push(tile.id);
                tiles_by_edge.entry(edge.chars().rev().collect()).or_insert(Vec::new()).push(tile.id);
            }
        });
        tiles_by_edge
}

fn find_corners(tiles_by_edge: &HashMap<String, Vec<usize>>) -> Vec<usize> {
    let counts = tiles_by_edge.values()
        .filter(|ids| ids.len() == 1)
        .fold(HashMap::<usize, u64>::new(), |mut acc, ids| {
            *acc.entry(ids[0]).or_default() += 1;
            acc
        });
    counts.iter()
        .filter(|&(&_id, &count)| count == 4)
        .map(|(id, _count)| *id)
        .collect()
}

fn construct_image(tiles_by_edge: &HashMap<String, Vec<usize>>, tiles: &HashMap<usize, Tile>, corner_id: usize) -> Vec<Vec<char>>{
    let mut corner = tiles.get(&corner_id).unwrap().clone();
    while [Direction::Left, Direction::Up].iter().any(|direction| corner.neighbour(tiles_by_edge, *direction).is_some()) {
        corner.rotate();
    }
    let width = (tiles.len() as f64).sqrt() as usize;
    let mut image = vec![vec![Tile::default(); width]; width];
    image[0][0] = corner;
    for i in 1..width {
        let above = &image[i-1][0];
        let neighbour_id = above.neighbour(tiles_by_edge, Direction::Down).unwrap();
        let mut tile = tiles.get(&neighbour_id).unwrap().clone();
        while tile.neighbour(tiles_by_edge, Direction::Up) != Some(above.id) {
            tile.rotate();
        }
        if tile.upper_edge() != above.lower_edge() {
            tile.flip_horizontal();
        }
        image[i][0] = tile;
    }
    for i in 0..width {
        for j in 1..width {
            let left = &image[i][j-1];
            let neighbour_id = left.neighbour(tiles_by_edge, Direction::Right).unwrap();
            let mut tile = tiles.get(&neighbour_id).unwrap().clone();
            while tile.neighbour(tiles_by_edge, Direction::Left) != Some(left.id) {
                tile.rotate();
            }
            if tile.left_edge() != left.right_edge() {
                tile.flip_vertical();
            }
            image[i][j] = tile;
        }
    }
    let mut final_image = vec![vec!['.'; 8 * width]; 8 * width];
    for i in 0..image.len() {
        for j in 0..image[i].len() {
            let tile = &image[i][j];
            for h in 1..9 {
                for w in 1..9 {
                    final_image[i * 8 + (h-1)][j * 8 + (w-1)] = tile.grid[h][w];
                }
            }
        }
    }
    final_image
}

fn num_sea_monsters(image: &Vec<Vec<char>>) -> u64 {
    let monster_offsets: Vec<(isize, isize)> = vec![(0, 0), (1, 1), (1, 4), (0, 5), (0, 6), (1, 7), (1, 10), (0, 11), (0, 12), (1, 13), (1, 16), (0, 17), (0, 18), (-1, 18), (0, 19)];
    let coordinates = image.iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter()
            .enumerate()
            .filter(|(_j, &c)| c == '#')
            .map(move |(j, _c)| (i as isize, j as isize))
        )
        .collect::<HashSet<(isize, isize)>>();
    let monster_coordinates = coordinates.iter()
        .map(|(i, j)| monster_offsets.iter()
            .map(|(offset_i, offset_j)| (i+offset_i, j+offset_j))
            .collect::<Vec<(isize, isize)>>()
        )
        .filter(|offset_coords| offset_coords.iter().all(|offset_coord| coordinates.contains(offset_coord)))
        .flatten()
        .collect::<HashSet<(isize, isize)>>();
    monster_coordinates.len() as u64
}

fn rotate(image: &mut Vec<Vec<char>>) {
    let length = image.len() as usize;
    for i in 0..length/2 {
        for j in i..length - i - 1 {
            let tmp = image[i][j];
            image[i][j] = image[j][length - 1 - i];
            image[j][length - 1 - i] = image[length - 1 - i][length - 1 - j];
            image[length - 1 - i][length - 1 - j] = image[length - 1 - j][i];
            image[length - 1 - j][i] = tmp;
        }
    }
}
fn flip_horizontal(image: &mut Vec<Vec<char>>) {
    for row in image {
        row.reverse();
    }
}
fn flip_vertical(image: &mut Vec<Vec<char>>) {
    image.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_test_input() -> String {
        fs::read_to_string("src/solutions/day20.test-input")
            .expect("No test-input file for day 20 found")
    }

    #[test]
    fn test_part_1() {
        let input = read_test_input();
        assert_eq!(part_1(&input), 20899048083289);
    }

    #[test]
    fn test_part_2() {
        let input = read_test_input();
        assert_eq!(part_2(&input), 273);
    }
}
