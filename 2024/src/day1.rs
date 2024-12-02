#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    todo!();
}

#[aoc(day1, part1, u32)]
pub fn part1(input: &Vec<u32>) -> usize {
    todo!();
}

#[aoc(day1, part2, u32)]
pub fn part2(input: &Vec<u32>) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2, input_generator};

    #[test]
    fn test_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(&input_generator(&input)), 11);
    }

}
