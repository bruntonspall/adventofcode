/*
* day8, Part 1.
* Oooh, sparse grid time, plus we have some maths to do here.
* Each antinode is defined as a mirror of any two nodes of the same kind.
* We're going to parse the list of nodes, and categorise them, so A -> [(1,1), (3,3)] etc.
* We're then going to go through each letter, and each pair of a given letter.
* For any given pair, we can work out the two antinodes, so if (5,5) and (7,8) are our pair has difference (2,3).
* We then look at a-difference and b+difference and those should be our antinodes.
* We only care about antinodes inside our grid, but I think we filter those out at the end.
*/

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::Coordinate;

pub struct Grid {
    map: HashMap<Coordinate, char>,
    width: i32,
    height: i32,
}

type GeneratorResult = Grid;
type RunResult = usize;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let mut nodes = HashMap::new();

    let maxx = input.lines().next().unwrap().len();
    let maxy = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                nodes.insert(Coordinate::new_usize(x, y), c);
            }
        }
    }
    Grid {
        map: nodes,
        width: maxx as i32,
        height: maxy as i32,
    }
}

/* Right, we want to invert the map, so that we have a nodes to coordinate hashmap as well */
pub fn invert(grid: &Grid) -> HashMap<char, Vec<Coordinate>> {
    let mut results: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for (coord, c) in grid.map.iter() {
        results.entry(*c).or_default().push(*coord);
    }
    results
}

/* For any pair, generate the antinodes */
pub fn generate_antinodes(
    first: Coordinate,
    second: Coordinate,
    maxx: i32,
    maxy: i32,
) -> Vec<Coordinate> {
    vec![first + (first - second), second + (second - first)]
        .iter()
        .filter(|c| c.x >= 0 && c.x < maxx && c.y >= 0 && c.y < maxy)
        .cloned()
        .collect()
}

pub fn find_antinodes_for_grid(grid: &Grid) -> HashSet<Coordinate> {
    let mut antinodes: HashSet<Coordinate> = HashSet::new();
    for coords in invert(grid).values() {
        for pair in coords.iter().combinations(2) {
            for c in generate_antinodes(*pair[0], *pair[1], grid.width, grid.height) {
                antinodes.insert(c);
            }
        }
    }
    antinodes
}

/* Ok, so for part 1, we create the inverted map, we then iterate through each pair, generating antinodes
 * We want to count the unique locations, so we can just insert them all into a set, and then count the total.
*/
#[aoc(day8, part1, RunResult)]
pub fn part1(grid: &GeneratorResult) -> RunResult {
    find_antinodes_for_grid(grid).len()
}

/*
 * day8, Part 2
 *
 */

#[aoc(day8, part2, RunResult)]
pub fn part2(grid: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;

    use crate::day8::*;

    #[test]
    fn test_generator() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let grid = input_generator(input);
        assert_eq!(12, grid.width);
        assert_eq!(12, grid.height);

        assert_eq_unordered!(
            grid.map.keys().collect(),
            vec![
                &Coordinate::new(8, 1),
                &Coordinate::new(5, 2),
                &Coordinate::new(7, 3),
                &Coordinate::new(4, 4),
                &Coordinate::new(6, 5),
                &Coordinate::new(8, 8),
                &Coordinate::new(9, 9),
            ]
        );
        assert_eq!(grid.map.get(&Coordinate::new(5, 2)), Some(&'0'));
        assert_eq!(grid.map.get(&Coordinate::new(8, 8)), Some(&'A'));

        let map = invert(&grid);
        assert_eq_unordered!(map.keys().collect(), vec![&'0', &'A']);
        assert_eq_unordered!(
            map.get(&'A').unwrap(),
            &vec![
                Coordinate::new(6, 5),
                Coordinate::new(8, 8),
                Coordinate::new(9, 9),
            ]
        );
    }

    #[test]
    fn test_generate_antinodes() {
        assert_eq!(
            generate_antinodes(Coordinate::new(5, 5), Coordinate::new(6, 7), 10, 10),
            vec![Coordinate::new(4, 3), Coordinate::new(7, 9)]
        );
        assert_eq!(
            generate_antinodes(Coordinate::new(6, 7), Coordinate::new(5, 5), 10, 10),
            vec![Coordinate::new(7, 9), Coordinate::new(4, 3)]
        );
        assert_eq!(
            generate_antinodes(Coordinate::new(1, 1), Coordinate::new(5, 5), 10, 10),
            vec![Coordinate::new(9, 9)]
        );
        assert_eq!(
            generate_antinodes(Coordinate::new(5, 5), Coordinate::new(9, 9), 10, 10),
            vec![Coordinate::new(1, 1)]
        );
    }

    #[test]
    fn test_example1() {
        let input = "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........";

        assert_eq!(part1(&input_generator(&input)), 4);
    }

    #[test]
    fn test_bounds() {
        let input = "aa..
.bb.
..cc
d.d.";
        let antinodes = find_antinodes_for_grid(&input_generator(&input));
        assert_eq_unordered!(
            antinodes.iter().cloned().collect(),
            vec![
                Coordinate::new(2, 0),
                Coordinate::new(0, 1),
                Coordinate::new(3, 1),
                Coordinate::new(1, 2)
            ]
        );
        assert_eq!(antinodes.len(), 4);
    }

    #[test]
    fn test_part1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(part1(&input_generator(&input)), 14);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
