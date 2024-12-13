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

use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::utils::Coordinate;

// Right, obstacles could be a map that goes from coordinate to boolean, and that works.
// But if we only care about existance, then we can probably use a HashSet, which is fast and easy.
pub type Obstacles = HashSet<Coordinate>;
#[derive(PartialEq, Debug, Clone)]
pub enum Facing {
    North,
    East,
    South,
    West,
}
pub type Guard = (Facing, Coordinate);

// We need the list of obstacles and also the guard position and facing
pub type GeneratorResult = (Obstacles, Guard);
pub type RunResult = usize;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let mut obstacles = Obstacles::new();
    let mut guard = (Facing::North, Coordinate::new(0, 0));
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert(Coordinate::new_usize(x, y));
                }
                '^' => guard = (Facing::North, Coordinate::new_usize(x, y)),
                '>' => guard = (Facing::East, Coordinate::new_usize(x, y)),
                'v' => guard = (Facing::South, Coordinate::new_usize(x, y)),
                '<' => guard = (Facing::West, Coordinate::new_usize(x, y)),
                _ => {}
            }
        }
    }
    (obstacles, guard)
}

/* Ok, we can now start our run process.
 * We're going to need to know when the guard leaves the space.
 * Now, I'm going to do something sightly wrong because we could get bounds when parsing.
 * But I actually think the input is going to have an obstacle on each edge somewhere,
 * so we can actually use unzip on our set to get streams of x and ys, and then get min and max from those */
pub fn find_bounds(obstacles: &Obstacles) -> (Coordinate, Coordinate) {
    obstacles.iter().fold(
        (Coordinate::new(1, 1), Coordinate::new(2, 2)),
        |current, obstacle| {
            (
                Coordinate::new(min(current.0.x, obstacle.x), min(current.0.y, obstacle.y)),
                Coordinate::new(max(current.1.x, obstacle.x), max(current.1.y, obstacle.y)),
            )
        },
    )
}

#[aoc(day6, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    let mut exited = false;
    let mut guard = input.1.clone();
    let mut seen = HashSet::new();
    let (Coordinate { x: minx, y: miny }, Coordinate { x: maxx, y: maxy }) = find_bounds(&input.0);
    while !exited {
        let next_square = match guard.0 {
            Facing::North => Coordinate::new(guard.1.x, guard.1.y - 1),
            Facing::East => Coordinate::new(guard.1.x + 1, guard.1.y),
            Facing::South => Coordinate::new(guard.1.x, guard.1.y + 1),
            Facing::West => Coordinate::new(guard.1.x - 1, guard.1.y),
        };
        if input.0.contains(&next_square) {
            guard.0 = match guard.0 {
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North,
            }
        } else if guard.1.x < minx || guard.1.x > maxx || guard.1.y < miny || guard.1.y > maxy {
            exited = true;
        } else {
            seen.insert(guard.1);
            guard.1 = next_square;
        }
    }
    seen.len()
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
        let (obstacles, guard) = input_generator(input);
        assert_eq!(Facing::North, guard.0);
        assert_eq!(Coordinate::new(4, 6), guard.1);
        assert!(obstacles.contains(&Coordinate::new(4, 0)));
        assert!(obstacles.contains(&Coordinate::new(9, 1)));
        assert!(obstacles.contains(&Coordinate::new(2, 3)));
    }

    #[test]
    fn test_find_bounds() {
        assert_eq!(
            (Coordinate::new(1, 1), Coordinate::new(3, 2)),
            find_bounds(&HashSet::from([
                Coordinate::new(1, 1),
                Coordinate::new(1, 2),
                Coordinate::new(3, 1)
            ]))
        );
        assert_eq!(
            (Coordinate::new(1, 1), Coordinate::new(4, 7)),
            find_bounds(&HashSet::from([
                Coordinate::new(1, 1),
                Coordinate::new(1, 7),
                Coordinate::new(3, 1),
                Coordinate::new(4, 3)
            ]))
        );
    }

    #[test]
    fn test_part1() {
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
        assert_eq!(part1(&input_generator(&input)), 41);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
