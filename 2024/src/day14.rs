/*
* day14, Part 1.
*/

use std::fmt;

type GeneratorResult = Vec<usize>;
type RunResult = usize;

/* Here's our coordinate, with an X and Y fields.  We're making sure it can be copied, clones, compared and printed */
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct ToroidalCoordinate {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/* Simple implementation, this might not be needed if I don't need any other methods */
impl ToroidalCoordinate {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl fmt::Display for ToroidalCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/* Here's our vector, which holds a change in X and Y fields (or Delta X and Delta Y).  We're making sure it can be copied, clones, compared and printed */
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Vector {
    pub dx: i32,
    pub dy: i32,
}

// Multiplying a Vector by a scalar is a lengthwise multiplication, i.e. (2,3)*3 should be (6,9)
impl std::ops::Mul<i32> for Vector {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            dx: self.dx * other,
            dy: self.dy * other,
        }
    }
}

/* We want to add a Vector to Coordinate to result in a new Coordinate, which is std::ops:Add to enable us to use the + symbol in the compiler */
impl std::ops::Add<Vector> for ToroidalCoordinate {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Self {
            x: (self.x + other.dx).rem_euclid(self.width),
            y: (self.y + other.dy).rem_euclid(self.height),
            width: self.width,
            height: self.height,
        }
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> GeneratorResult {
    todo!();
}

#[aoc(day14, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
}

/*
 * day14, Part 2
 *
 */

#[aoc(day14, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day14::*;

    #[test]
    fn test_coordinates() {
        let coord = ToroidalCoordinate::new(2, 4, 11, 7);
        let v = Vector { dx: 2, dy: -3 };
        // Test simple multiplication
        assert_eq!(v * 4, Vector { dx: 8, dy: -12 });

        // Test robot positions
        for (time, (targetx, targety)) in vec![
            (1, (4, 1)),
            (2, (6, 5)),
            (3, (8, 2)),
            (4, (10, 6)),
            (5, (1, 3)),
        ] {
            assert_eq!(
                coord + (v * time),
                ToroidalCoordinate {
                    x: targetx,
                    y: targety,
                    height: 7,
                    width: 11
                }
            );
        }
    }

    #[test]
    fn test_generator() {}

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(&input_generator(&input)), 0);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
