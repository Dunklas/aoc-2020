use std::{fmt, cmp, clone, marker};

#[derive(fmt::Debug, cmp::PartialEq, cmp::Eq, clone::Clone, marker::Copy)]
pub struct CartesianCoordinate {
    pub x: usize,
    pub y: usize,
}
impl CartesianCoordinate {
    pub fn new(x: usize, y: usize) -> Self {
        CartesianCoordinate{
            x, y
        }
    }
}
