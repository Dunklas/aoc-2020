use std::{str, marker, fmt, cmp};
use crate::grid::coordinate::CartesianCoordinate;

#[derive(fmt::Debug)]
pub struct Grid<T> {
    internal: Vec<Vec<T>>,
    coordinates: Vec<CartesianCoordinate>
}

impl<T> Grid<T> {
    fn new(vec2d: Vec<Vec<T>>) -> Self {
        let mut coordinates: Vec<CartesianCoordinate> = Vec::new();
        for (y, row) in vec2d.iter().enumerate() {
            for (x, _element) in row.iter().enumerate() {
                coordinates.push(CartesianCoordinate::new(x,y));
            }
        }
        Grid{
            internal: vec2d,
            coordinates,
        }
    }
}

#[derive(fmt::Debug)]
pub struct ParseGridError;
#[derive(fmt::Debug)]
pub struct GridOutOfBoundsError;

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        self.internal.iter().for_each(|line| {
            line.iter().for_each(|c| {
                string.push_str(&format!("{}", c));
            });
            string.push_str("\n");
        });
        write!(f, "{}", string)
    }
}

impl<T: str::FromStr + fmt::Display> str::FromStr for Grid<T> {
    type Err = ParseGridError;    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<T>> = Vec::new();
        for line in s.lines() {
            let mut row: Vec<T> = Vec::new(); 
            for c in line.chars() {
                row.push(c.to_string().parse()
                    .or(Err(ParseGridError))?);
            }
            grid.push(row);
        }
        Ok(Grid::new(grid))
    }
}

impl<T: marker::Copy + cmp::PartialEq> Grid<T> {
    pub fn at(self: &Self, pos: &CartesianCoordinate) -> Result<T, GridOutOfBoundsError> {
        let row = match self.internal.get(pos.y) {
            Some(s) => s,
            None => return Err(GridOutOfBoundsError)
        };
        let element = match row.get(pos.x) {
            Some(s) => s,
            None => return Err(GridOutOfBoundsError)
        };
        Ok(*element)
    }
    pub fn adjacent_to(self: &Self, src_pos: &CartesianCoordinate, target: T) -> usize {
        let src_x = src_pos.x as isize;
        let src_y = src_pos.y as isize;
        vec![(src_x-1, src_y), (src_x+1, src_y), (src_x, src_y-1), (src_x, src_y+1), (src_x-1, src_y-1), (src_x+1, src_y-1), (src_x-1, src_y+1), (src_x+1, src_y+1)]
            .iter()
            .filter(|(x,y)| *x >= 0 && *y >= 0)
            .map(|(x,y)| CartesianCoordinate::new(*x as usize,*y as usize))
            .filter(|pos| {
                match self.at(pos) {
                    Ok(val) => val == target,
                    Err(_e) => false,
                }
            })
            .count()
    }
    pub fn number_of(self: &Self, element: T) -> usize {
        self.coordinates.iter()
            .filter_map(|pos| self.at(pos).ok())
            .filter(|x| *x == element)
            .count()
    }
}

impl<T: marker::Copy> Grid<T> {
    pub fn set(self: &mut Self, pos: &CartesianCoordinate, new: T) -> Result<(), GridOutOfBoundsError> {
        let row = match self.internal.get_mut(pos.y) {
            Some(row) => row,
            None => return Err(GridOutOfBoundsError), 
        };
        match row.get(pos.x) {
            None => return Err(GridOutOfBoundsError),
            _ => {},
        };
        row[pos.x] = new;
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn width(self: & Self) -> Result<usize, GridOutOfBoundsError> {
        match self.internal.get(0) {
            Some(s) => {
                Ok(s.len())
            },
            None => {
                Err(GridOutOfBoundsError)
            }
        }
    }
    pub fn height(self: & Self) -> usize {
        self.internal.len()
    }
    pub fn coordinates(self: & Self) -> Vec<CartesianCoordinate> {
        self.coordinates.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width() {
        let grid: Grid<char> = "ABCDE\n".parse().unwrap();
        assert_eq!(grid.width().unwrap(), 5);
    }

    #[test]
    fn height() {
        let grid: Grid<char> = "abcde\n".parse().unwrap();
        assert_eq!(grid.height(), 1);
    }

    #[test]
    fn coordinates() {
        let grid: Grid<char> = "ab\nAB".parse().unwrap();
        assert_eq!(grid.coordinates(), vec![
            CartesianCoordinate::new(0,0), CartesianCoordinate::new(1,0),
            CartesianCoordinate::new(0,1), CartesianCoordinate::new(1,1)
        ]);
    }

    #[test]
    fn at_pos() {
        let grid: Grid<char> = "abcde\nfghij".parse().unwrap();
        assert_eq!(grid.at(&CartesianCoordinate::new(0,0)).unwrap(), 'a');
        assert_eq!(grid.at(&CartesianCoordinate::new(4,0)).unwrap(), 'e');
        assert_eq!(grid.at(&CartesianCoordinate::new(0,1)).unwrap(), 'f');
        assert_eq!(grid.at(&CartesianCoordinate::new(4,1)).unwrap(), 'j');
    }

    #[test]
    fn adjacent_to_left_right() {
        let grid: Grid<char> = "#.##.##.##\n".parse().unwrap();
        assert_eq!(grid.adjacent_to(&CartesianCoordinate::new(4,0), '#'), 2);
        assert_eq!(grid.adjacent_to(&CartesianCoordinate::new(4,0), '.'), 0);
    }

    #[test]
    fn adjacent_to_up_down() {
        let grid: Grid<char> = "#\n.\n#".parse().unwrap();
        assert_eq!(grid.adjacent_to(&CartesianCoordinate::new(0,1), '#'), 2);
        assert_eq!(grid.adjacent_to(&CartesianCoordinate::new(0,0), '.'), 1);
    }

    #[test]
    fn adjacent_diagonal() {
        assert_eq!("#..\n...\n...".parse::<Grid<char>>().unwrap().adjacent_to(&CartesianCoordinate::new(1,1), '#'), 1);
        assert_eq!("..#\n...\n...".parse::<Grid<char>>().unwrap().adjacent_to(&CartesianCoordinate::new(1,1), '#'), 1);
        assert_eq!("...\n...\n#..".parse::<Grid<char>>().unwrap().adjacent_to(&CartesianCoordinate::new(1,1), '#'), 1);
        assert_eq!("...\n...\n..#".parse::<Grid<char>>().unwrap().adjacent_to(&CartesianCoordinate::new(1,1), '#'), 1);
        assert_eq!("###\n#.#\n###".parse::<Grid<char>>().unwrap().adjacent_to(&CartesianCoordinate::new(1,1), '#'), 8);
    }

    #[test]
    fn set() {
        let mut grid: Grid<char> = "###".parse().unwrap();
        let result = grid.set(&CartesianCoordinate::new(1,0), '.');
        let element = grid.at(&CartesianCoordinate::new(1,0)).unwrap();
        assert!(result.is_ok());
        assert_eq!(element, '.');
    }

    #[test]
    fn number_of() {
        let grid: Grid<char> = "###.".parse().unwrap();
        assert_eq!(grid.number_of('#'), 3);
        assert_eq!(grid.number_of('.'), 1);
    }
}
