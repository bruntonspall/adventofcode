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

use std::collections::HashMap;

use crate::utils::Coordinate;

type GeneratorResult = HashMap<Coordinate, char>;
type RunResult = u32;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let mut nodes = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                nodes.insert(Coordinate::new_usize(x, y), c);
            }
        }
    }
    nodes
}

/* Right, we want to invert the map, so that we have a nodes to coordinate hashmap as well */
pub fn invert(hm: GeneratorResult) -> HashMap<char, Vec<Coordinate>> {
    let mut results: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for (coord, c) in hm {
        results.entry(c).or_default().push(coord);
    }
    results
}

/* For any pair, generate the antinodes */
pub fn generate_antinodes(first: Coordinate, second: Coordinate) -> Vec<Coordinate> {
    vec![first + (first - second), second + (second - first)]
}


/* Ok, so for part 1, we create the inverted map, we then iterate through each pair, generating antinodes 
 * We want to count the unique locations, so we can just insert them all into a set, and then count the total.
*/
#[aoc(day8, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
}

/*
 * day8, Part 2
 *
 */

#[aoc(day8, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;

    use crate::day8::*;

    #[test]
    fn test_generator() {
        let input = "..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........";

        let hm = input_generator(input);
        assert_eq_unordered!(
            hm.keys().collect(),
            vec![
                &Coordinate::new(3, 1),
                &Coordinate::new(0, 2),
                &Coordinate::new(4, 3),
                &Coordinate::new(8, 4),
                &Coordinate::new(5, 5),
                &Coordinate::new(2, 6),
                &Coordinate::new(6, 7),
            ]
        );
        assert_eq!(hm.get(&Coordinate::new(3, 1)), Some(&'#'));
        assert_eq!(hm.get(&Coordinate::new(5, 5)), Some(&'a'));
        assert_eq!(hm.get(&Coordinate::new(6, 7)), Some(&'A'));

        let map = invert(hm);
        assert_eq_unordered!(map.keys().collect(), vec![&'a', &'A', &'#']);
        assert_eq_unordered!(
            map.get(&'#').unwrap(),
            &vec![
                Coordinate::new(2, 6),
                Coordinate::new(0, 2),
                Coordinate::new(3, 1),
            ]
        );
    }

    #[test]
    fn test_generate_antinodes() {
        assert_eq!(
            generate_antinodes(Coordinate::new(5, 5), Coordinate::new(6, 7)),
            vec![Coordinate::new(4, 3), Coordinate::new(7, 9)]
        );
        assert_eq!(
            generate_antinodes(Coordinate::new(6, 7), Coordinate::new(5, 5)),
            vec![Coordinate::new(7, 9), Coordinate::new(4, 3)]
        );
    }

    #[test]
    fn test_part1() {
        let input = "..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........";

        assert_eq!(part1(&input_generator(&input)), 14);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
