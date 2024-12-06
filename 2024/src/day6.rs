/*
* day6, Part 1.
*
* Ok, it's our first proper grid.  On day 4 I talked about grids and said there was a different between a sparse grid and a tight grid.
* Day 4 we went for a Vec of Vecs to give a grid, but this time we have obstacles at specific points, so sparse grids are in vogue
* We'll read the input, fill a map with locations of obstacles, and then we can consider our main loop.
* In traditional games programming, we would just have an infinite loop that exits if the game is quit.
* Our equivalent here might be:
* while (not_exited) {
* if obstacles[guard.target()] { guard.turnright() }
* if ourofbounds(guard.target()) { not_exited = false }
* seen.add(guard.move())
* }
* This loop would check the next location, if there's an obstacle, change facing. 
* It needs to check if it's leaving the arena, and signal an exit
* then it moves guard forward 1, and adds the place it was to the list of tiles walked on
*
* We could turn that into a fold I suspect, starting with a state that is the location, heading, and seen list, and each 
* iteration would change the state and return a new state and seen list.
* But the game dev in me prefers the latter, and I think it's more extendable for a part 2
*/

use std::collections::HashSet;

pub type Coordinate = (usize,usize);
// Right, obstacles could be a map that goes from coordinate to boolean, and that works.
// But if we only care about existance, then we can probably use a HashSet, which is fast and easy.
pub type Obstacles = HashSet<Coordinate>;
#[derive(PartialEq, Debug)]
pub enum Facing {
    North,
    East,
    South,
    West,
}
pub type Guard = (Facing, Coordinate);

// We need the list of obstacles and also the guard position and facing
pub type GeneratorResult = (Obstacles,Guard);
pub type RunResult = u32;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let mut obstacles = Obstacles::new();
    let mut guard = (Facing::North,(0,0));
    for (y,line) in input.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '#' => { obstacles.insert((x,y)); },
                '^' => { guard = (Facing::North, (x,y))}
                '>' => { guard = (Facing::East, (x,y))}
                'v' => { guard = (Facing::South, (x,y))}
                '<' => { guard = (Facing::West, (x,y))}
                _ => {}
            }
        }
    }
    (obstacles, guard)
}

#[aoc(day6, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
}

/*
 * day6, Part 2
 *
 */

#[aoc(day6, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    #[test]
    fn test_generator() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let (obstacles,guard) = input_generator(input);
        assert_eq!(Facing::North, guard.0);
        assert_eq!((4,6), guard.1);
        assert!(obstacles.contains(&(4,0)));
        assert!(obstacles.contains(&(9,1)));
        assert!(obstacles.contains(&(2,3)));
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
