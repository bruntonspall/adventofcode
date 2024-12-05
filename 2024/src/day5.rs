/*
* day5, Part 1.
* Right, read a list of predicates, followed by some input data, and check all the predicates match... simples!
* Reading the text carefully, we can probably simplify this all down.  I was originally considering creating
* curried functions, and building a list of them, and then applying them to the input data.
* But actually in this case, the test is always the same, for each number, find all the rules with the start number
* and then process all the successor numbers against the rules.  If there's an error, break out and invalidate the row.
* We can hardcode the processing itself, so the data structure can almost certainly just be tuples for the pairs, and
* then a vector for each update.
*/

use std::collections::HashMap;

type GeneratorResult = (Vec<(u32, u32)>, Vec<Vec<u32>>);
type RunResult = u32;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> GeneratorResult {
    // Right parsing, we're going to parse lines until an empty line, and then we parse the results separately.

    let left: Vec<(u32, u32)> = input
        .lines()
        .take_while(|line| line != &"")
        .map(|line| match line.split("|").collect::<Vec<&str>>() {
            pair => (
                pair[0].parse::<u32>().expect("Numbers only"),
                pair[1].parse::<u32>().expect("Numbers only"),
            ),
            _ => (0, 0),
        })
        .collect();
    let right = input
        .lines()
        .skip_while(|line| line != &"")
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u32>().expect("Numbers only"))
                .collect()
        })
        .collect();
    (left, right)
}

/* Right, to do part one, there's a number of ways to do it.  The easiest, but most inefficient, is 
 * to go through the input list, one at a time.
 * Because we're looking for rules that accidentally have the wrong order, for example 75,97 when there's a rule 97|75
 * we want to walk backwards through the pages, and check that no page number violates a rule that it must come after.
 * That however requires walking both lists repeatedly, which is horrribly inefficient.
 * We could possibly preprocess the rules into a map, which would mean we could look up the current number, and see 
 * all the antecedents allowed.
 * This would turn rules:
75|29
75|53
97|29
97|53
75|47
97|75
75|61
75|13
 * into something more like this:
 * 75 -> 29,53,47,61,13
 * 97 -> 29,53,75
 * 
 * This would mean that we walk the input list, and for each number, check whether anything after the input number is in
 * the list of numbers that should be after.
 * That's cleaner and more efficient.  I also have a feeling there must be a helper to build a map from pairs of numbers...
 * Update: Ok, HashMap::from can take a list of key,value, but what it will do is replace duplicates rather than append them, 
 * so we'll have to do that by hand
 */
pub fn map_from_pairs(pairs: Vec<(u32, u32)>) -> HashMap<u32,Vec<u32>> {
    let mut lookuptable:HashMap<u32,Vec<u32>> = HashMap::new();
    for (key,value) in pairs {
        lookuptable.entry(key).or_default().push(value);
    }
    lookuptable
}

#[aoc(day5, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
}

/*
 * day5, Part 2
 *
 */

#[aoc(day5, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    fn test_generator() {
        let input = "47|53
97|13
97|61

75,47,61,53,29
75,29,13";
        let expected = (
            vec![(47, 53), (97, 13), (97, 61)],
            vec![vec![75, 47, 61, 53, 29], vec![75, 29, 13]],
        );
        assert_eq!(expected, input_generator(input));
    }

    #[test]
    fn test_map_from_pairs() {
        let input = vec![(1,2), (1,3), (2,3), (3,4)];
        let output = map_from_pairs(input);
        assert_eq!(output.get(&1), Some(&vec![2,3]));
        assert_eq!(output.get(&2), Some(&vec![3]));
        assert_eq!(output.get(&3), Some(&vec![4]));
        assert_eq!(output.get(&4), None);
    }

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
