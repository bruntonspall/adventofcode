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
    x: i32,
    y: i32
}

/* Simple implementation, this might not be needed if I don't need any other methods */
impl Coordinate {
    fn new(x: i32, y:i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

/* We want to add Coordinates together, which is std::ops:Add to enable us to use the + symbol in the compiler */
impl std::ops::Add for Coordinate {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
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
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize
}

impl Grid {
    fn new(source: &Vec<Vec<char>>) -> Self {
        // We're going to initialise from the collection becase we want to know the length up front
        let mut grid = Grid {
            grid: source.clone(), // We're making a copy here, this might make more sense to be a borrow
            height: source.len(),
            width: source[0].len()
        };
        return grid;
    }

    fn get(self: &Self, c: Coordinate) -> char {
        // Number conversions in Rust make me sad... lots of "expect" here.
        if c.x >= i32::try_from(self.width).expect("Width is way too high") || c.x < 0 || c.y >= i32::try_from(self.height).expect("Height is way too high") || c.y < 0 { 
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
    Grid::new(&input.lines().map(|x|{
        x.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>())
}

#[aoc(day4, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
}

/*
 * Day 4, Part 2
 *
 */

#[aoc(day4, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day4::*;

    #[test]
    fn test_coordinate() {
        assert_eq!(Coordinate::new(1, 2).x, 1);
        assert_eq!(Coordinate::new(1, 2).y, 2);
        assert_eq!(Coordinate::new(1, 2)+Coordinate::new(3, 4), Coordinate::new(4, 6));
        assert_eq!(Coordinate::new(5, 5)+Coordinate::new(-1, -1), Coordinate::new(4, 4));
    }

    #[test]
    fn test_grid() {
        let test_grid = Grid::new(&vec![vec!['a', 'b', 'c'],vec!['d', 'e','f']]);
        assert_eq!(test_grid.get(Coordinate::new(0,0)), 'a');
        assert_eq!(test_grid.get(Coordinate::new(1,0)), 'b');
        assert_eq!(test_grid.get(Coordinate::new(0,1)), 'd');
        assert_eq!(test_grid.get(Coordinate::new(2,1)), 'f');
        assert_eq!(test_grid.get(Coordinate::new(3,1)), '.');
        assert_eq!(test_grid.get(Coordinate::new(1,3)), '.');
        assert_eq!(test_grid.get(Coordinate::new(-3,999)), '.');
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
        assert_eq!(grid.get(Coordinate::new(0, 0)),'M');
        assert_eq!(grid.get(Coordinate::new(1, 0)),'M');
        assert_eq!(grid.get(Coordinate::new(2, 0)),'M');
        assert_eq!(grid.get(Coordinate::new(3, 0)),'S');
        assert_eq!(grid.get(Coordinate::new(0, 1)),'M');
        assert_eq!(grid.get(Coordinate::new(1, 1)),'S');
        assert_eq!(grid.get(Coordinate::new(9, 9)),'X');
    }

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
