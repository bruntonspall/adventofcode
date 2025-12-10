use std::collections::HashSet;

use crate::utils::Coordinate;

fn neighbours(coord: &Coordinate) -> HashSet<Coordinate> {
    let mut res = HashSet::new();
    for y in coord.y - 1..coord.y + 2 {
        for x in coord.x - 1..coord.x + 2 {
            if !(x == coord.x && y == coord.y) {
                res.insert(Coordinate::new(x, y));
            }
        }
    }
    res
}

fn count_neighbours(grid: &HashSet<Coordinate>, location: &Coordinate) -> usize {
    neighbours(location).intersection(grid).count()
}

fn parse_grid(input: &str) -> HashSet<Coordinate> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '@' => Some(Coordinate::new(x as i32, y as i32)),
                _ => None,
            })
        })
        .collect::<HashSet<_, _>>()
}

pub fn calculate_part1(input: &str) -> usize {
    let grid = parse_grid(input);
    grid.iter()
        .filter(|coord| count_neighbours(&grid, coord) < 4)
        .count()
}

pub fn calculate_part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use assert_unordered::assert_eq_unordered;

    use crate::{
        day4::{calculate_part1, count_neighbours, neighbours, parse_grid},
        utils::Coordinate,
    };

    #[test]
    fn test_find_neighbours() {
        for (expected, input) in [
            (
                vec![
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .into_iter()
                .map(|(x, y)| Coordinate::new(x, y))
                .collect::<Vec<Coordinate>>(),
                Coordinate::new(0, 0),
            ),
            (
                vec![
                    (4, 7),
                    (5, 7),
                    (6, 7),
                    (4, 8),
                    (6, 8),
                    (4, 9),
                    (5, 9),
                    (6, 9),
                ]
                .into_iter()
                .map(|(x, y)| Coordinate::new(x, y))
                .collect(),
                Coordinate::new(5, 8),
            ),
        ] {
            let neighbours: Vec<Coordinate> = neighbours(&input).into_iter().collect();
            assert_eq_unordered!(neighbours, expected);
        }
    }

    #[test]
    fn test_parse_grid() {
        for (expected, input) in [
            (vec![(0, 0), (1, 1)], "@.\n.@"),
            (vec![(1, 0), (0, 1)], ".@..\n@..."),
            (vec![(2, 0), (0, 1), (2, 2)], "..@.\n@...\n..@."),
        ] {
            let grid: HashSet<Coordinate> = parse_grid(input);
            assert_eq_unordered!(
                grid,
                expected
                    .into_iter()
                    .map(|(x, y)| Coordinate::new(x, y))
                    .collect()
            );
        }
    }

    #[test]
    fn test_count_neighbours() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = parse_grid(input);
        assert_eq!(count_neighbours(&grid, &Coordinate::new(0, 0)), 2);
        assert_eq!(count_neighbours(&grid, &Coordinate::new(2, 0)), 3);
        assert_eq!(count_neighbours(&grid, &Coordinate::new(0, 2)), 4);
        assert_eq!(count_neighbours(&grid, &Coordinate::new(1, 2)), 7);
    }

    #[test]
    fn test_part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(13, calculate_part1(input));
    }
}
