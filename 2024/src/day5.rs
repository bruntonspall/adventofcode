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
