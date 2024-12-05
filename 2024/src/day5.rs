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
pub fn map_from_pairs(pairs: &Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    let mut lookuptable: HashMap<u32, Vec<u32>> = HashMap::new();
    for (key, value) in pairs {
        lookuptable.entry(*key).or_default().push(*value);
    }
    lookuptable
}

pub fn is_valid_update(update: &Vec<u32>, table: &HashMap<u32, Vec<u32>>) -> bool {
    let mut candidate_iter = update.iter();
    while let Some(candidate) = candidate_iter.next() {
        // Check that this number(candidate) is followed by any numbers that must follow it
        // println!();
        // print!("Checking {} - ", candidate);
        let mut check_iter = candidate_iter.clone();
        while let Some(check) = check_iter.next() {
            // print!(" {}", check);
            if !table.get(candidate).unwrap_or(&vec![]).contains(check) {
                // print!(" - X");
                return false;
            }
        }
    }
    return true;

}

pub fn find_valid<'a>(
    input: &'a Vec<Vec<u32>>,
    table: &HashMap<u32, Vec<u32>>,
) -> Vec<&'a Vec<u32>> {
    // println!("Pages: {:?}", input);
    // println!("Table: {:?}", table);

    let correct_pages: Vec<&Vec<u32>> = input
        .iter()
        .filter(|update| is_valid_update(update, table))
        .collect();
    // println!("Correct Pages: {:?}", correct_pages);
    return correct_pages;
}

#[aoc(day5, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    let table = map_from_pairs(&input.0);
    let correct_pages = find_valid(&input.1, &table);
    correct_pages
        .iter()
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

/*
 * day5, Part 2
 *
 * Oh my.  Ok, so firstly, we'll invert find valid to get us just the invalid results.
 * We then need to "fix_result" for a single result, which will put it in order.
 * I honestly don't know how to approach this one in any way that is even slightly efficient.
 * We could go through as we did before, and when we find a number that doesn't fit, we push it
 * to the back, and try the next number etc.
 * But I'm not convinced that will actually work, as it wont handle some weird situations.
 * For example, given [1,2,3,4,5] and {1:4,5}, {2:1,3,4,5}, {3:1,2,3,4,5}, {4:5}, {5:}
 * It would see 1, not be able to place it and put it to the back, giving [2,3,4,5,1]
 * It would see 2, and that would be fine to place, giving [3,4,5,1]
 * It would see 3, and that would be fine to place, giving [4,5,1]
 * It would see 4, which can't have 1 in, so would go to the back, giving [5,1,4]
 * It would see 5, which it can't see giving [1,4,5]
 * It would see 1, and that would be fine to place, giving [4,5]
 * It would see 4, and that would be fine to place, giving [5]
 * and it would see 5 and place it, for a total of 2,3,1,4,5
 *
 * Huh, maybe it would work... Let's try that
 * I think the biggest issue is probably catching an infinite loop, so I wonder if we use two lists,
 * candidates and rejected.  It would iterate through candidates, pushing them to rejected if rejected.
 * When placing a number, it would then combine candidates and rejected and go again.
 * If candidates is ever empty with rejected not empty, then we need to panic and error?
 */

fn fix_result<'a>(updates: &'a Vec<u32>, table: &HashMap<u32, Vec<u32>>) -> &'a Vec<u32> {
    let mut candidates = updates;
    let rejected:Vec<u32> = vec![];
    let results:Vec<u32> = vec![];
    updates
}

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
        let input = vec![(1, 2), (1, 3), (2, 3), (3, 4)];
        let output = map_from_pairs(&input);
        assert_eq!(output.get(&1), Some(&vec![2, 3]));
        assert_eq!(output.get(&2), Some(&vec![3]));
        assert_eq!(output.get(&3), Some(&vec![4]));
        assert_eq!(output.get(&4), None);
    }

    #[test]
    fn test_find_valid() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let (pairs, updates) = input_generator(input);
        let valid = find_valid(&updates, &map_from_pairs(&pairs));
        assert_eq!(valid.len(), 3);
        assert_eq!(valid[0], &vec![75, 47, 61, 53, 29]);
        assert_eq!(valid[1], &vec![97, 61, 53, 29, 13]);
        assert_eq!(valid[2], &vec![75, 29, 13]);
    }

    #[test]
    fn test_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part1(&input_generator(&input)), 143);
    }

    #[test]
    fn test_fix_results() {
        let updates = vec![1, 2, 3, 4, 5];
        let table = map_from_pairs(&vec![
            (1, 4),
            (1, 5),
            (2, 1),
            (2, 3),
            (2, 4),
            (2, 5),
            (3, 1),
            (3, 2),
            (3, 3),
            (3, 4),
            (3, 5),
            (4, 5),
        ]);
        assert_eq!(fix_result(&updates, &table), &vec![2, 3, 1, 4, 5]);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
