/*
* Day 4, Part 1 - Word search

Ok, we're at our first grid based day of the advent of code.
There are two data structures we can use here, a simple one or a slightly more complex one.
In the simple one, we simply use a vector of vectors, so [[1,2],[3,4]] style, and index into it using grid[y][x].
The downside to that is that we need to handle asking for x or y coordinates that are outside the bounds.
The more complex one is to use a Map structure that stores the glyph for each coordinate, so grid[(x,y)].
The advantage of a map structure is we can use a get_or_default fetch, and return a known null glyph ('.') for coordinate that isn't matched.
Generally speaking, Maps are better for sparse grids, where we have a small number of items.  In this case we have a dense structure, so I think I'm going to go for the Vector of Vectors approach.

We're going to wrap that in a Grid struct, with a getAt function that handles out of bounds checking.

On the actual algorithm, I think the simplest option is to go through the entire grid, character by character, lookng for 'X' glyphs.
When we find one, we'll then start a search in all 8 cardinal directions, moving the coordinates by 1 each time, and search for M, A and S.  We'll return the number we found, and then continue the search.

I note that we're going to want to think about coordinates and maybe add them together (because a move north is like adding (0,-1) to (x,y)), and so we may want a coordinate structure as well.
*/

use std::char;

/* Here's our coordinate, with an X and Y fields.  We're making sure it can be copied, clones, compared and printed */
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

/* Simple implementation, this might not be needed if I don't need any other methods */
impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn new_usize(x: usize, y: usize) -> Self {
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

/* Ok, and our grid will hold our array of arrays.  Note I said vector above, but actually, this isn't going to
 * change size once initialised, so an array is probably more efficient.
 * We can't define the size in the struct because we don't know it at compile time, only at runtime
 *
 * Update: Rust is really pedantic and wont let us create fixed sized arrays of chars, because different chars are different sizes.
 * It feels less efficient, but we're going to use Vector of Vectors instead.
 * */
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

type GeneratorResult = Grid;
type RunResult = u32;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> GeneratorResult {
    Grid::new(
        &input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
}

#[aoc(day4, part1, RunResult)]
pub fn part1(grid: &GeneratorResult) -> RunResult {
    // Ok, we're going to go through the entire grid, looking for 'X' characters.
    // When we find one, we're then going to try all 8 cardinal directions, and we're going to iterate 3 steps in each direction, looking for 'M', 'A' and 'S' respectively.
    // I'm going to use a mutable counter for this, because while I could probably do all of that in a number of map/fold left style lines, I think some for loops might be easier to read (for me at least).
    let directions = [
        Coordinate::new(-1, -1),
        Coordinate::new(0, -1),
        Coordinate::new(1, -1),
        Coordinate::new(-1, 0),
        Coordinate::new(1, 0),
        Coordinate::new(-1, 1),
        Coordinate::new(0, 1),
        Coordinate::new(1, 1),
    ];
    let mut found = 0;
    for y in 0..grid.height {
        // println!("");
        // print!("Starting row {}: ", y);
        for x in 0..grid.width {
            if grid.get(Coordinate::new_usize(x, y)) == 'X' {
                // print!("X");
                let c = Coordinate::new_usize(x, y);
                for direction in directions {
                    let mut foundhere = true;
                    for (step, letter) in [(1, 'M'), (2, 'A'), (3, 'S')] {
                        if grid.get(c + (direction * step)) != letter {
                            foundhere = false
                        }
                    }
                    if foundhere {
                        found += 1;
                    }
                }
                // } else {
                //     print!(".");
            }
        }
    }
    return found;
}

/*
 * Day 4, Part 2 - I can see stars everywhere
 *
 * Ok, this looks horrendous at first, but actually it's just a variation of the previous approach.
 * Instead of searching for X's and then walking 3 steps in a direction, we're now going to look for 'A's first.
 * We're then going to look in the 4 diagonal directions, for specificallly, 'M' and 'S' tokens.
 * Again, I could simplify, but I'm  not sure I want to.
 *
 *
 * Update...
 *
 * Oh inteersting, where the M and the S are doesn't matter, providing they are opposite.  That's an interesting twist.
 * So if we find an 'A', we don't know if we're looking for an 'M' or an 'S'.
 * Easy exhaustice search is to look at all four corners for an M and then look in opposite corner for an S.
 * To find the opposite corner, I suspect we can multiply the coordinate by -1, becuase (1,1)'s opposite is -1,-1, and 1,-1 is opposite -1,1...
 */

#[aoc(day4, part2, RunResult)]
pub fn part2(grid: &GeneratorResult) -> RunResult {
    let directions = [
        Coordinate::new(-1, -1),
        Coordinate::new(1, -1),
        Coordinate::new(-1, 1),
        Coordinate::new(1, 1),
    ];
    let mut found = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(Coordinate::new_usize(x, y)) == 'A' {
                let c = Coordinate::new_usize(x, y);
                let mut foundhere = 0;
                for direction in directions {
                    // So, I've inverted the booleans. I think that A && B == 'A || 'B, but I'm not sure
                    // enough that it wont match MAM or .AM for example, so this makes more sense.  Start at
                    // Finding an A isn't enough, must also find the M and the S in the directions.

                    // Update, that didn't work, it counted each diagonal, not each X.  We need to more clearly match
                    // only a 5 character sequence.
                    // The rules says that we need to match each MAS can be written forwards and backwards.
                    // We might consider counting letters?  2 M's and 2'S's?  No that would match a MAM/SAS cross.
                    // I think the right model might be to look in each direction for an M and validate that there's an
                    // S opposite, which should match only 2 possible answers.  We need both to match, so maybe instead of
                    // a true/false, we count, and only accept if there are 2 instead of just 1.
                    if grid.get(c + direction) == 'M' && grid.get(c + (direction * -1)) == 'S' {
                        foundhere += 1
                    }
                }
                if foundhere == 2 {
                    found += 1;
                }
            }
        }
    }
    return found;
}

#[cfg(test)]
mod tests {
    use crate::day4::*;

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

    #[test]
    fn test_generator() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let grid = input_generator(input);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.get(Coordinate::new(0, 0)), 'M');
        assert_eq!(grid.get(Coordinate::new(1, 0)), 'M');
        assert_eq!(grid.get(Coordinate::new(2, 0)), 'M');
        assert_eq!(grid.get(Coordinate::new(3, 0)), 'S');
        assert_eq!(grid.get(Coordinate::new(0, 1)), 'M');
        assert_eq!(grid.get(Coordinate::new(1, 1)), 'S');
        assert_eq!(grid.get(Coordinate::new(9, 9)), 'X');
    }

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(&input_generator(&input)), 18);
    }

    #[test]
    fn test_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part2(&input_generator(&input)), 9);
    }
}
