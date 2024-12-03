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

pub fn high_varience(report: &Vec<i32>) -> bool {
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
}

pub fn rise_or_fall(report: &Vec<i32>) -> bool {
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
}

#[aoc(day2, part1, usize)]
pub fn part1(input: &Vec<Vec<i32>>) -> usize {
    input
        .iter() // For each report card
        .filter(|report| high_varience(*report)) // And then filter out those with high varience
        .filter(|report| rise_or_fall(*report)) // Filter out those that don't consistently rise or fall
        .count()
}

/*
 * Day 2, Part 2 - Oh My!
 *
 * Reading this, the report is safe if any one of the numbers if removed from the report
 * At first I thought that maybe I could use the same filters to ignore one error, but that's both hard and of course may not fix it.
 * If you have numbers a b c, where a to b is an error and b to c is an error, then we'd need to skip 2 errors.
 * Instead, I think we need to create permutations of the input report, with each number missing, plus the original, and then mark it as safe if any of the permutations are safe
 *
 * The any predicate should work for this, but the hard bit for me is finding a way to generate that list of permutations.
 * I was going to consider iterators or generators, but honestly, sometimes just brute force it!
 */

pub fn permutate_list(input: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut v: Vec<Vec<i32>> = Vec::new();
    for (index, _) in input.iter().enumerate() {
        let mut n = input.clone();
        n.remove(index);
        v.push(n);
    }
    return v;
}

#[aoc(day2, part2, usize)]
pub fn part2(input: &Vec<Vec<i32>>) -> usize {
    /* This is exactly what was in part1, but with an any applied to the post generator set */
    /* Ok, take 2, copying over the bits from part 1 was hard, so we're going to refactor part 1 into predicates to make it cleaner */
    input
        .iter() // For each report card
        .filter(|report| {
            // Filter out report cards where
            permutate_list(report) // For each of the report permutations
                .iter()
                .any(|perm| high_varience(perm) && rise_or_fall(perm))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day2::{high_varience, input_generator, part1, part2, permutate_list, rise_or_fall};

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
    fn test_permutate_list() {
        assert_eq!(
            permutate_list(&vec![1, 2, 3]),
            vec![vec![2, 3], vec![1, 3], vec![1, 2]]
        );
        assert_eq!(
            permutate_list(&vec![2, 4, 6, 8]),
            vec![vec![4, 6, 8], vec![2, 6, 8], vec![2, 4, 8], vec![2, 4, 6]]
        );
    }

    #[test]
    fn test_high_varience() {
        assert!(high_varience(&vec![7, 6, 4, 2, 1]));
        assert!(!high_varience(&vec![1, 2, 7, 8, 9]));
        assert!(!high_varience(&vec![9, 7, 6, 2, 1]));
        assert!(high_varience(&vec![1, 3, 2, 4, 5]));
        assert!(!high_varience(&vec![8, 6, 4, 4, 1]));
        assert!(high_varience(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_increase_or_decrease() {
        assert!(rise_or_fall(&vec![7, 6, 4, 2, 1]));
        assert!(rise_or_fall(&vec![1, 2, 7, 8, 9]));
        assert!(rise_or_fall(&vec![9, 7, 6, 2, 1]));
        assert!(!rise_or_fall(&vec![1, 3, 2, 4, 5]));
        assert!(!rise_or_fall(&vec![8, 6, 4, 4, 1]));
        assert!(rise_or_fall(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!(part2(&input_generator(&input)), 4);
    }
}
