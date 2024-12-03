/*
* Day 3, Part 1.
*  Ok, so today looks like it's mostly a parsing challange.
* I've stretched myself the last few with trying to do things in a functional way, and while I probably could build some kind of iterator here, I think I'm just going to go for the simple solution which is to use a regular expression
* I say simple, I've not done this in rust before.
*
* The regular expression is fairly easy, "mul\((\d+),(\d+)\)" is what we're looking for.  With grouping rules, that should consume the word mul and the open parenthesis, and then return the first digit, consume the comma, return the second digit and then consume the closing parenthesis
*
*It looks like Rust doesn't have native regex support, so we'll crate in the regex library, which looks like the standard one using cargo add regex.
Regex looks like it operates by creating a new regex string, and then we can use the captures_iter to iterate through the captures using the extract method bring them out into a match object and the groups.  The example has a handy example of naming parts of the group, so `for (_, [group1, group2]) in re.captures_iter(line).map(|c|c.extract()) {}` should do the trick
*/

use regex::Regex;

type GeneratorResult = Vec<(u32, u32)>;
type RunResult = u32;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex compilation shouldn't fail!");
    /* In this case, we're expecting just one really big line, so return a vector of multiplications */
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| {
            (
                a.parse::<u32>().expect("Failed to parse number"),
                b.parse::<u32>().expect("Failed to parse number"),
            )
        })
        .collect()
}

#[aoc(day3, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    /* Well this looks easy, I wonder what the catch is... */
    input.iter().map(|(a, b)| a * b).sum()
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
    fn test_generator() {
        assert_eq!(input_generator(&"xmul(2,4)x"), vec![(2, 4)]);
        assert_eq!(input_generator(&"xmul(2,4)xmul(4,8)"), vec![(2, 4), (4, 8)]);
        assert_eq!(
            input_generator(&"xmul(2,4)xthen(mul(11,8)"),
            vec![(2, 4), (11, 8)]
        );
    }

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)";
        assert_eq!(part1(&input_generator(&input)), 161);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
