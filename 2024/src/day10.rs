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

use std::collections::{HashMap, HashSet};

use crate::day4::{Coordinate, Grid};

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

/* Ok, before we can do a breadth first search, we need to be able to get valid neighbours for going downhill.
 * a getneighbours for the grid will use a simple match to determine next valid token and then search 4 cardinal
 * directions for that token */
pub fn next_token(token: char) -> char {
    match token {
        '9' => '8',
        '8' => '7',
        '7' => '6',
        '6' => '5',
        '5' => '4',
        '4' => '3',
        '3' => '2',
        '2' => '1',
        '1' => '0',
        _ => 'X',
    }
}

impl Grid {
    pub fn valid_neighbours(self: &Self, loc: Coordinate) -> Vec<Coordinate> {
        let neighbours = vec![
            Coordinate::new(loc.x, loc.y - 1),
            Coordinate::new(loc.x + 1, loc.y),
            Coordinate::new(loc.x, loc.y + 1),
            Coordinate::new(loc.x - 1, loc.y),
        ];
        let mut valid_neighbours = Vec::new();
        let looking_for = next_token(self.get(loc));
        for neighbour in neighbours {
            if self.get(neighbour) == looking_for {
                valid_neighbours.push(neighbour);
            }
        }
        return valid_neighbours;
    }
}

/* Ok, we can write a breadth first search here.  In this case, I think we'll take a starting point, and return only a list
 * of the valid trailheads (so only '0').
 *
 * Rough algorithm is:
 * start with a mutable list 'open' of places to examine (filled initially with the start location)
 * while open has items:
 *   take the next location from open
 *   if next location is '0', add the next location to list of trailheads
 *   Get valid neighbours from the next location, and push them onto the back of open
 * finally, return the list of trailheads
 */
impl Grid {
    pub fn bfs_trailheads(self: &Self, start: Coordinate) -> Vec<Coordinate> {
        let mut open: Vec<Coordinate> = Vec::new();
        let mut trailheads = Vec::new();
        open.push(start);
        while let Some(next) = open.pop() {
            if self.get(next) == '0' {
                trailheads.push(next);
            } else {
                open.extend(self.valid_neighbours(next));
            }
        }
        trailheads
    }

    pub fn calculate_trailheads(self: &Self) -> (HashSet<(Coordinate, Coordinate)>,HashMap<Coordinate, usize>) {
        let mut scores = HashMap::new();
        let mut connections = HashSet::new();
        let summit_points: Vec<Coordinate> = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(x, cell)| '9' == **cell)
                    .map(move |(x, cell)| Coordinate::new_usize(x, y))
            })
            .collect();
        println!("Testing all summits");
        for c in summit_points {
            print!("Summit {} :", c);
            for trailhead in self.bfs_trailheads(c) {
                connections.insert((c,trailhead));
                print!("  TH{}=", trailhead);
                let score = scores.entry(trailhead).or_insert(0);
                *score += 1;
                print!("{}", score);
            }
            println!("");
        }
        (connections, scores)
    }
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
    use assert_unordered::assert_eq_unordered;

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
        assert_eq!('8', grid.get(Coordinate { x: 0, y: 0 }));
        assert_eq!('9', grid.get(Coordinate { x: 1, y: 0 }));
        assert_eq!('7', grid.get(Coordinate { x: 0, y: 1 }));
        assert_eq!('2', grid.get(Coordinate { x: 7, y: 7 }));
    }

    #[test]
    fn test_valid_neighbours() {
        let input = "0123
1234
8765
9876";
        let grid = input_generator(input);
        assert_eq!(
            grid.valid_neighbours(Coordinate { x: 0, y: 3 }),
            vec![Coordinate { x: 0, y: 2 }, Coordinate { x: 1, y: 3 }]
        );
        assert_eq!(
            grid.valid_neighbours(Coordinate { x: 0, y: 2 }),
            vec![Coordinate { x: 1, y: 2 }]
        );
        assert_eq!(
            grid.valid_neighbours(Coordinate { x: 1, y: 3 }),
            vec![Coordinate { x: 1, y: 2 }, Coordinate { x: 2, y: 3 }]
        );
    }

    #[test]
    fn test_bfs_search() {
        let input = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        let grid = input_generator(input);
        let trailheads = grid.bfs_trailheads(Coordinate { x: 0, y: 6 });
        assert_eq!(trailheads, vec![Coordinate { x: 3, y: 0 }]);
    }

    #[test]
    fn test_calculate_scores() {
        let input = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        let grid = input_generator(input);
        let (connections, trailheads) = grid.calculate_trailheads();
        assert_eq!(trailheads.len(), 1);
        assert_eq!(trailheads.get(&Coordinate { x: 3, y: 0 }), Some(&2));
        assert_eq!(connections.len(), 2);
        assert_eq_unordered!(connections, HashSet::from([(Coordinate { x: 0, y: 6 }, Coordinate { x: 3, y: 0 }),(Coordinate { x: 6, y: 6 }, Coordinate { x: 3, y: 0 })]));

        let grid = input_generator("..90..9
...1.98
...2..7
6543456
765.987
876....
987....");
        let (connections, trailheads) = grid.calculate_trailheads();
        assert_eq!(trailheads.len(), 1);
        assert_eq!(trailheads.get(&Coordinate { x: 3, y: 0 }), Some(&13));
        assert_eq!(connections.len(), 4);
        // assert_eq_unordered!(connections, HashSet::from([(Coordinate { x: 0, y: 6 }, Coordinate { x: 3, y: 0 }),(Coordinate { x: 6, y: 6 }, Coordinate { x: 3, y: 0 })]));

        let grid = input_generator("10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01");
        let (connections, trailheads) = grid.calculate_trailheads();
        assert_eq!(trailheads.len(), 2);
        assert_eq!(trailheads.get(&Coordinate { x: 1, y: 0 }), Some(&1));
        assert_eq!(trailheads.get(&Coordinate { x: 5, y: 6 }), Some(&2));
        assert_eq!(connections.len(), 3);
        // assert_eq_unordered!(connections, HashSet::from([(Coordinate { x: 0, y: 6 }, Coordinate { x: 3, y: 0 }),(Coordinate { x: 6, y: 6 }, Coordinate { x: 3, y: 0 })]));
        let grid = input_generator("89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732");
        let (connections, trailheads) = grid.calculate_trailheads();
        assert_eq!(trailheads.len(), 9);
        /* This isn't true, it wont have a score of 9, it will have a score of 20 because there are 20 different ways to get there */
        // assert_eq!(trailheads.get(&Coordinate { x: 2, y: 0 }), Some(&9));
        assert_eq!(trailheads.get(&Coordinate { x: 2, y: 0 }), Some(&20));
        assert_eq!(connections.len(), 36);
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
