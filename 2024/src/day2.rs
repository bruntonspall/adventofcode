/*
 * Day 2, Part 1.
 *
 */

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    todo!()
}

#[aoc(day2, part1, i32)]
pub fn part1(input: &Vec<(i32, i32)>) -> i32 {
    todo!()
}

/*
 * Ok, Part 2, this time we're counting the number of occorences in the right list instead of multiplying directly.
 * We can use a counter to count the right side, and then we can map the left list by multiplying it by the counts on the right.
 */
#[aoc(day2, part2, i32)]
pub fn part2(input: &Vec<(i32, i32)>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day2::{input_generator, part1, part2};

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!(part1(&input_generator(&input)), 2);
    }

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
