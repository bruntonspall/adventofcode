use counter::Counter;
use std::str::FromStr;

/*
 * Day 1, Part 1.
 *
 * Ok, we have to take a list of pairs of numbers, split them apart, sort them, and then pair them again to calculate the differences.
 * Our generator will handle taking each line and generating a tuple of two numbers.
 * Because we want to sort the lists, we're going to inefficiently use two iterators, one taking all the first of the pair, and one taking all of the second of the pair.  We'll push those into a vector, sort it, and then we can zip them up with an iterator over the pair, finding the absolute difference between them.
 *
 * Ok, parsing integers from strings is a bit of faff.  In this case we know we have good inputs, so I've used expect to handle the Try.  That consumes an OK and panics on an Err.  A better thing here would be to have a match that handles the error appropriately, but in this case, panicking is probably the right move.
 *
 * Ok, so lets do part 1.  We're going to iterate over the list of numbers, pushing each item into a pair of vectors, and then we'll sort the vectors, and then zip them and total up the differences.
 */

pub fn pair_from_str(line: &str) -> (i32, i32) {
    let numpair = line.split_whitespace().collect::<Vec<&str>>();
    if numpair.len() == 2 {
        return (
            i32::from_str(numpair[0]).expect("Should be a digit"),
            i32::from_str(numpair[1]).expect("Should be a digit"),
        );
    }
    panic!("Should be a pair");
}
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input.lines().map(|line| pair_from_str(line)).collect()
}

#[aoc(day1, part1, i32)]
pub fn part1(input: &Vec<(i32, i32)>) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    // let mut total: i32 = 0;
    for pair in input {
        left.push(pair.0);
        right.push(pair.1);
    }
    left.sort();
    right.sort();
    // ok, we can simplyfy this down to a simple map and then a sum
    //
    // for pair in left.iter().zip(right) {
    //     total += if *pair.0 > pair.1 {
    //         pair.0 - pair.1
    //     } else {
    //         pair.1 - pair.0
    //     }
    // }
    // total
    left.iter()
        .zip(right)
        .map(|pair| (pair.0 - pair.1).abs())
        .sum()
}

/*
 * Ok, Part 2, this time we're counting the number of occorences in the right list instead of multiplying directly.
 * We can use a counter to count the right side, and then we can map the left list by multiplying it by the counts on the right.
 */
#[aoc(day1, part2, i32)]
pub fn part2(input: &Vec<(i32, i32)>) -> i32 {
    let mut counter: Counter<i32, i32> = Counter::new();
    // We have to use for each here, rather than map, because we're after the side effect rather than the result
    input.iter().for_each(|pair| counter[&pair.1] += 1);
    input
        .iter()
        .map(|pair| counter.get(&pair.0).unwrap_or(&0) * pair.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{input_generator, pair_from_str, part1, part2};

    #[test]
    fn test_pair_from_str() {
        /*
         * It's my first time parsing digits from strings, this will become normal, but lets test this first time
         */
        assert_eq!(pair_from_str("1 2"), (1, 2));
        assert_eq!(pair_from_str("1 -2"), (1, -2));
        assert_eq!(pair_from_str("2 1"), (2, 1));
        assert_eq!(pair_from_str("1   3"), (1, 3));
    }

    #[test]
    fn test_generator() {
        /*
         * It's my first time parsing digits from strings, so lets test the generator
         */
        assert_eq!(
            input_generator(
                "1 2
3 4"
            ),
            vec![(1, 2), (3, 4)]
        );
        assert_eq!(
            input_generator(
                "1 -2
-3 4"
            ),
            vec![(1, -2), (-3, 4)]
        );
    }
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

    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part2(&input_generator(&input)), 31);
    }
}
