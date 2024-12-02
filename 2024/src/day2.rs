/*
 * Day 2, Part 1.
 *
 * Ok, so we're reading rows of data, each with 5 sequential levels.
 * Ah ha, doing something I sometimes forget to do, the actual data has a varying number of levels, not a fixed number, so we'll need to think about that
 * I suspect that means that we have a Vector of Vectors of u32's, so each are variable length.
 *
 * We then need to filter the list based on a predicate that can be ascertained from reviewing the list.
 * The second rpedicate, any two adjacant level differs by at least one and at most three is easy to cehck, as the predicate relies on no outside knowledge
 * But the levels are either all increasing or all decreasing requires either keeping state from item to item, or running over the list twice.
 *
 * My gut feel is to do a check to mark as unsafe if "direction changes" or if the change is outside of 1..3
 *
 */
pub fn list_from_str(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|num| num.parse::<i32>().expect("Must be numbers"))
        .collect()
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(list_from_str).collect()
}

#[aoc(day2, part1, usize)]
pub fn part1(input: &Vec<Vec<i32>>) -> usize {
    input
        .iter() // For each report card
        .filter(|report| {
            // Filter it down if
            report
                .windows(2) // For each pair of values
                .all(
                    // They all...
                    |pair| {
                        let diff = pair[0].abs_diff(pair[1]);
                        diff >= 1 && diff <= 3 // Have a difference of 1-3
                    },
                )
        }) // And then filter out
        .filter(|report| {
            report // For each report
                .windows(2) // For each pair of values
                .fold(0, |dir, pair| {
                    if pair[0] < pair[1] && (dir == 0 || dir == 1) {
                        1
                    }
                    // If the last pair was increasing or the start
                    else if pair[0] > pair[1] && (dir == 0 || dir == -1) {
                        -1
                    }
                    // Of if the last pair was decreasing or the start
                    else {
                        -99
                    } // Otherwise signal an error
                })
                != -99 // Check for any errors
        })
        .count()
}

/*
 * Day 2, Part 2
 */
#[aoc(day2, part2, usize)]
pub fn part2(input: &Vec<Vec<i32>>) -> usize {
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
