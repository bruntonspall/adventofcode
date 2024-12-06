/*
* day6, Part 1.
*/

type GeneratorResult = Vec<u32>;
type RunResult = u32;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> GeneratorResult {
    todo!();
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
