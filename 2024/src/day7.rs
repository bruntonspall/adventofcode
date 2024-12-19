/*
* day7, Part 1.
*/

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Equation {
    total: u64,
    args: Vec<u64>,
}

pub enum Operand {
    MUL,
    ADD,
}

type GeneratorResult = Vec<Equation>;
type RunResult = usize;

pub fn total(target: u64, args: &Vec<u64>) -> u32 {
    pub fn total_rec(target: u64, running_total: u64, args: &Vec<u64>, index: usize) -> u32 {
        // Our base case, make sure it returns when we run out of numbers
        if index == args.len() {
            return if running_total == target { 1 } else { 0 };
        }
        // Handle addition, which is running_total + args[index] and repeat for next index
        total_rec(target, running_total + args[index], args, index + 1)
            + total_rec(target, running_total * args[index], args, index + 1)
    }
    total_rec(target, args[0], args, 1)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .lines()
        .map(|line| {
            let sum_s = line
                .split(':')
                .next()
                .unwrap()
                .parse::<u64>()
                .expect("Must be a number");
            let args = line
                .split(':')
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().expect("Must be numbers"))
                .collect();
            Equation {
                total: sum_s as u64,
                args: args,
            }
        })
        .collect()
}

#[aoc(day7, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    input
        .iter()
        .map(|equation| {
            if total(equation.total, &equation.args) > 0 {
                equation.total
            } else {
                0
            }
        })
        .sum::<u64>() as usize
}

/*
 * day7, Part 2
 *
 */

#[aoc(day7, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day7::*;

    #[test]
    fn test_generator() {
        assert_eq!(
            vec![Equation {
                total: 7,
                args: vec![1, 2, 3]
            }],
            input_generator("7: 1 2 3")
        );
        assert_eq!(
            vec![
                Equation {
                    total: 190,
                    args: vec![10, 19]
                },
                Equation {
                    total: 3267,
                    args: vec![81, 40, 27]
                },
            ],
            input_generator(
                "190: 10 19
3267: 81 40 27"
            )
        );
    }

    #[test]
    fn test_total() {
        // We're going to change these tests when we add multiplication
        // Check base case, in this case we should reach 5
        assert_eq!(1, total(5, &vec![5]));

        // 1+2 and 1*2
        assert_eq!(1, total(2, &vec![1, 2]));

        // 1+2+3=6, 1+2*3=9, 1*2+3=5, 1*2*3=6
        assert_eq!(2, total(6, &vec![1, 2, 3]));
        assert_eq!(1, total(5, &vec![1, 2, 3]));
        assert_eq!(1, total(9, &vec![1, 2, 3]));

        // 1+2+4=7, 1+2*4=12, 1*2+4=6, 1*2*4=8
        assert_eq!(1, total(6, &vec![1, 2, 4]));
        assert_eq!(1, total(7, &vec![1, 2, 4]));
        assert_eq!(1, total(8, &vec![1, 2, 4]));
        assert_eq!(1, total(12, &vec![1, 2, 4]));
    }

    #[test]
    fn test_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part1(&input_generator(&input)), 3749);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
