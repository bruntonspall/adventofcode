use std::str::FromStr;

/*
 * Day 1, Part 1.
 * 
 * Ok, we have to take a list of pairs of numbers, split them apart, sort them, and then pair them again to calculate the differences.
 * Our generator will handle taking each line and generating a tuple of two numbers.
 * Because we want to sort the lists, we're going to inefficiently use two iterators, one taking all the first of the pair, and one taking all of the second of the pair.  We'll push those into a vector, sort it, and then we can zip them up with an iterator over the pair, finding the absolute difference between them.
 * 
 * Ok, parsing integers from strings is a bit of faff.  In this case we know we have good inputs, so I've used expect to handle the Try.  That consumes an OK and panics on an Err.  A better thing here would be to have a match that handles the error appropriately, but in this case, panicking is probably the right move.
 */

 pub fn pair_from_str(line: &str) -> (i32, i32) {
    let numpair = line.split_whitespace().collect::<Vec<&str>>(); 
    if numpair.len() == 2 {
        return (i32::from_str(numpair[0]).expect("Should be a digit"), i32::from_str(numpair[1]).expect("Should be a digit"))
    }
    panic!("Should be a pair");
 }
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input.lines().map(|line|pair_from_str(line)).collect()
}    

#[aoc(day1, part1, u32)]
pub fn part1(input: &Vec<(i32,i32)>) -> usize {
    todo!();
}

#[aoc(day1, part2, u32)]
pub fn part2(input: &Vec<(i32,i32)>) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day1::{input_generator, pair_from_str, part1, part2};

    #[test]
    fn test_pair_from_str() {
        /*
         * It's my first time parsing digits from strings, this will become normal, but lets test this first time
         */
        assert_eq!(pair_from_str("1 2"), (1,2));
        assert_eq!(pair_from_str("1 -2"), (1,-2));
        assert_eq!(pair_from_str("2 1"), (2,1));
        assert_eq!(pair_from_str("1   3"), (1,3));
    }

    #[test]
    fn test_generator() {
        /*
         * It's my first time parsing digits from strings, so lets test the generator
         */
        assert_eq!(input_generator("1 2
3 4"), vec![(1,2),(3,4)]);
assert_eq!(input_generator("1 -2
-3 4"), vec![(1,-2),(-3,4)]);
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

}
