/*
* day7, Part 1.
*/

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Equation {
    total: u32,
    args: Vec<u32>
}

pub enum Operand {
    MUL,
    ADD,
}

type GeneratorResult = Vec<Equation>;
type RunResult = usize;

pub fn total(running_total:u32, args: &Vec<u32>, index: usize) -> u32 {
    // Our base case, make sure it returns when we run out of numbers
    if index == args.len() {
        return running_total;
    }
    // Handle addition, which is running_total + args[index] and repeat for next index
    total(running_total+args[index], args, index+1)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input.lines().map(|line|{
        let sum_s = line.split(':').next().unwrap().parse::<u32>().expect("Must be a number");
        let args = line.split(':').skip(1).next().unwrap().split_whitespace().map(|num|num.parse::<u32>().expect("Must be numbers")).collect();
        Equation{total: sum_s as u32, args: args}
    }).collect()
}

#[aoc(day7, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
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
        assert_eq!(vec![Equation{total:7, args:vec![1,2,3]}], input_generator("7: 1 2 3"));
        assert_eq!(vec![
            Equation{total:190, args:vec![10,19]},
            Equation{total:3267, args:vec![81,40,27]},
            ], input_generator("190: 10 19
3267: 81 40 27"));
    }

    #[test]
    fn test_total() {
        // We're going to change these tests when we add multiplication
        // Check base case
        assert_eq!(0, total(0,&vec![],0));
        assert_eq!(5, total(0,&vec![5],0));
        assert_eq!(7, total(0,&vec![5,2],0));
        assert_eq!(11, total(0,&vec![5,2,4],0));

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
