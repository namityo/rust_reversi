use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "座標:{},{}", self.x, self.y)
    }
}
