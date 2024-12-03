/*
 * Day 3, Part 1.
 *
 *
 */

type GeneratorResult = (i32, i32);
type RunResult = usize;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> GeneratorResult {
    (0, 0)
}

#[aoc(day3, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
}

/*
 * Day 2, Part 2
 *
 */

#[aoc(day3, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)";
        assert_eq!(part1(&input_generator(&input)), 0);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
