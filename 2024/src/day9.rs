/*
* day9, Part 1.
*/

type GeneratorResult = Vec<usize>;
type RunResult = usize;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> GeneratorResult {
    todo!();
}

#[aoc(day9, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
}

/*
 * day9, Part 2
 *
 */

#[aoc(day9, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day9::*;

    #[test]
    fn test_generator() {}

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
