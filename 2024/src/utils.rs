use std::fmt;

/* Here's our coordinate, with an X and Y fields.  We're making sure it can be copied, clones, compared and printed */
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

/* Simple implementation, this might not be needed if I don't need any other methods */
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn new_usize(x: usize, y: usize) -> Self {
        Self {
            x: i32::try_from(x).expect("X out of bounds"),
            y: i32::try_from(y).expect("Y out of bounds"),
        }
    }
}

/* We want to add Coordinates together, which is std::ops:Add to enable us to use the + symbol in the compiler */
impl std::ops::Add for Coordinate {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Multiplying a coordinate by a scalar is a lengthwise multiplication, i.e. (2,3)*3 should be (6,9)
impl std::ops::Mul<i32> for Coordinate {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub struct Grid {
    pub grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(source: &Vec<Vec<char>>) -> Self {
        // We're going to initialise from the collection becase we want to know the length up front
        let grid = Grid {
            grid: source.clone(), // We're making a copy here, this might make more sense to be a borrow
            height: source.len(),
            width: source[0].len(),
        };
        return grid;
    }

    pub fn get(self: &Self, c: Coordinate) -> char {
        // Number conversions in Rust make me sad... lots of "expect" here.
        if c.x >= i32::try_from(self.width).expect("Width is way too high")
            || c.x < 0
            || c.y >= i32::try_from(self.height).expect("Height is way too high")
            || c.y < 0
        {
            '.'
        } else {
            self.grid[usize::try_from(c.y).expect("")][usize::try_from(c.x).expect("")]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::*;

    #[test]
    fn test_coordinate() {
        assert_eq!(Coordinate::new(1, 2).x, 1);
        assert_eq!(Coordinate::new(1, 2).y, 2);
        assert_eq!(
            Coordinate::new(1, 2) + Coordinate::new(3, 4),
            Coordinate::new(4, 6)
        );
        assert_eq!(
            Coordinate::new(5, 5) + Coordinate::new(-1, -1),
            Coordinate::new(4, 4)
        );
        let (x,y):(usize,usize) = (1,2);
        assert_eq!(Coordinate::new(1, 2), Coordinate::new_usize(x,y))
    }

    #[test]
    fn test_grid() {
        let test_grid = Grid::new(&vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
        assert_eq!(test_grid.get(Coordinate::new(0, 0)), 'a');
        assert_eq!(test_grid.get(Coordinate::new(1, 0)), 'b');
        assert_eq!(test_grid.get(Coordinate::new(0, 1)), 'd');
        assert_eq!(test_grid.get(Coordinate::new(2, 1)), 'f');
        assert_eq!(test_grid.get(Coordinate::new(3, 1)), '.');
        assert_eq!(test_grid.get(Coordinate::new(1, 3)), '.');
        assert_eq!(test_grid.get(Coordinate::new(-3, 999)), '.');
    }
}
