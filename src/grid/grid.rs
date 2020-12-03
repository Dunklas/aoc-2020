use std::{str, marker, fmt};

pub struct Grid<T: str::FromStr + marker::Copy + fmt::Display> {
    internal: Vec<Vec<T>>
}
#[derive(fmt::Debug)]
pub struct ParseGridError;
#[derive(fmt::Debug)]
pub struct GridOutOfBoundsError;

impl<T: str::FromStr + marker::Copy + fmt::Display> str::FromStr for Grid<T> {
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
        Ok(Self{
            internal: grid
        })
    }
}

impl<T: str::FromStr + marker::Copy + fmt::Display> Grid<T> {
    pub fn at(self: &Self, x: usize, y: usize) -> Result<T, GridOutOfBoundsError> {
        let row = match self.internal.get(y) {
            Some(s) => s,
            None => return Err(GridOutOfBoundsError)
        };
        let element = match row.get(x) {
            Some(s) => s,
            None => return Err(GridOutOfBoundsError)
        };
        Ok(*element)
    }

    pub fn width(self: &Self) -> Result<usize, GridOutOfBoundsError> {
        match self.internal.get(0) {
            Some(s) => {
                Ok(s.len())
            },
            None => {
                Err(GridOutOfBoundsError)
            }
        }
    }

    pub fn height(self: &Self) -> usize {
        self.internal.len()
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
    fn at_pos() {
        let grid: Grid<char> = "abcde\nfghij".parse().unwrap();
        assert_eq!(grid.at(0,0).unwrap(), 'a');
        assert_eq!(grid.at(4,0).unwrap(), 'e');
        assert_eq!(grid.at(0,1).unwrap(), 'f');
        assert_eq!(grid.at(4,1).unwrap(), 'j');
    }
}
