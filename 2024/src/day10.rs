/*
* day10, Part 1.
* Ok, it's our routine pathfinding examples.  
* In this case, we have to find all the trailheads (slots with a 0) that leads in a continuous upwards trail to a 9.
* We're counting how many summits we can reach from each trailhead, and counting them.
* 
* First of all, we have a dense grid, we're going to want to use a vector of vectors, but we're also going to want
* some intermediate data structures, we need to know the locations of all the summits and the locations of all the
* trail heads in a list as well.
* Given that it's easy and cheap to do, we'll write a function to get these from the grid rather than calculate at scan time.
*
* In typical pathfinding method, I think we're actually going to go from end back to beginning rather than beginning to end.
* For each summit, we'll do a breadth first search, looking for single steps down, adding those to a list of open nodes, and
* if we find a trailhead, we'll increment the nunmber of summits that can be reached.  
* We know we can't go anywhere from the trailhead.
*
* Finally, we used a grid already on day4.  I can probably import Coordinate and Grid from day4, and I'll refactor into a utility
* at teh end of this day
*/

use crate::day4::{Grid, Coordinate};

type GeneratorResult = Grid;
type RunResult = u32;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> GeneratorResult {
    Grid::new(
        &input
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
}

#[aoc(day10, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
}

/*
 * day10, Part 2
 *
 */

#[aoc(day10, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test]
    fn test_generator() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let grid = input_generator(input);
        assert_eq!(8, grid.width);
        assert_eq!(8, grid.height);
        assert_eq!('8', grid.get(Coordinate{x:0, y:0}));
        assert_eq!('9', grid.get(Coordinate{x:1, y:0}));
        assert_eq!('7', grid.get(Coordinate{x:0, y:1}));
        assert_eq!('2', grid.get(Coordinate{x:7, y:7}));
    }

    #[test]
    fn test_part1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part1(&input_generator(&input)), 36);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
