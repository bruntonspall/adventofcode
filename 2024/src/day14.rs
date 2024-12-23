/*
* day14, Part 1.
*/

use std::{collections::HashMap, fmt};

use itertools::{Itertools, Position};
use regex::Regex;

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

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Robot {
    position: ToroidalCoordinate,
    velocity: Vector,
}

impl Robot {
    pub fn position_at(self: &Self, time: i32) -> Self {
        return Robot {
            position: self.position + (self.velocity * time),
            velocity: self.velocity,
        };
    }
}

pub fn count_quadrants(robots: &Vec<Robot>, partition_x: i32, partition_y: i32) -> Vec<Option<u8>> {
    robots
        .iter()
        .map(|r| {
            if r.position.x < partition_x && r.position.y < partition_y {
                Some(1)
            } else if r.position.x > partition_x && r.position.y < partition_y {
                Some(2)
            } else if r.position.x < partition_x && r.position.y > partition_y {
                Some(3)
            } else if r.position.x > partition_x && r.position.y > partition_y {
                Some(4)
            } else {
                None
            }
        })
        .collect::<Vec<Option<u8>>>()
}

type GeneratorResult = Vec<Robot>;
type RunResult = usize;

static mut WORLD_WIDTH: i32 = 101;
static mut WORLD_HEIGHT: i32 = 103;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let re =
        Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").expect("Regex compilation shouldn't fail!");
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y, dx, dy])| Robot {
            position: ToroidalCoordinate {
                x: x.parse::<i32>().unwrap(),
                y: y.parse::<i32>().unwrap(),
                width: unsafe { WORLD_WIDTH },
                height: unsafe { WORLD_HEIGHT },
            },
            velocity: Vector {
                dx: dx.parse::<i32>().unwrap(),
                dy: dy.parse::<i32>().unwrap(),
            },
        })
        .collect()
}

#[aoc(day14, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    unsafe {
        count_quadrants(
            &input.iter().map(|r| r.position_at(100)).collect(),
            WORLD_WIDTH / 2,
            WORLD_HEIGHT / 2,
        )
        .iter()
        .flatten()
        .counts()
        .values()
        .product()
    }
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
    use assert_unordered::assert_eq_unordered;

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
    fn test_generator() {
        // Changing the constants is unsafe for multi-threaded code
        unsafe {
            WORLD_WIDTH = 11;
            WORLD_HEIGHT = 7;
        }
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3";
        assert_eq!(
            input_generator(input),
            vec![
                Robot {
                    position: ToroidalCoordinate {
                        x: 0,
                        y: 4,
                        width: 11,
                        height: 7
                    },
                    velocity: Vector { dx: 3, dy: -3 }
                },
                Robot {
                    position: ToroidalCoordinate {
                        x: 6,
                        y: 3,
                        width: 11,
                        height: 7
                    },
                    velocity: Vector { dx: -1, dy: -3 }
                },
            ]
        )
    }

    #[test]
    fn test_sample_input() {
        // Changing the constants is unsafe for multi-threaded code
        unsafe {
            WORLD_WIDTH = 11;
            WORLD_HEIGHT = 7;
        }

        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let robots = input_generator(input);
        let endstate: Vec<Robot> = robots.iter().map(|r| r.position_at(100)).collect();
        #[rustfmt::skip]
        assert_eq_unordered!(
            endstate.iter().map(|robot|(robot.position.x, robot.position.y))
            .collect(), 
            vec![
                (6,0), (6,0), (9,0),
                (0,2),
                (1,3),(2,3),
                (5,4),
                (3,5),(4,5),(4,5),
                (1,6),(6,6),
        ]);
        unsafe {
            let counts = count_quadrants(&endstate, WORLD_WIDTH / 2, WORLD_HEIGHT / 2);
            assert_eq_unordered!(
                counts,
                vec![
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(1),
                    None,
                    None,
                    None,
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(3),
                    Some(4)
                ]
            )
        }
    }

    #[test]
    fn test_part1() {
        unsafe {
            WORLD_WIDTH = 11;
            WORLD_HEIGHT = 7;
        }

        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part1(&input_generator(&input)), 12);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
