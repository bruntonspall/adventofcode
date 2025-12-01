fn parse_input(_input: &str) -> Vec<i32> {
    _input
        .lines()
        .map(|line| line.trim())
        .map(|s| match s {
            _ if s.starts_with('R') => s[1..].parse::<i32>().expect("Failed to parse integer"),
            _ if s.starts_with('L') => -s[1..].parse::<i32>().expect("Failed to parse integer"),
            _ => panic!("Invalid direction"),
        })
        .collect()
}

fn rotate_dial(start: i32, turn: i32) -> i32 {
    // (start + turn + 100) % 100
    (start + turn).rem_euclid(100)
}

fn execute_instructions(start: i32, instructions: &Vec<i32>) -> Vec<i32> {
    let mut dial = start;
    let mut results = Vec::new();
    for &instruction in instructions {
        dial = rotate_dial(dial, instruction);
        results.push(dial);
    }
    results
}

fn execute_instructions_part2(start: i32, instructions: &Vec<i32>) -> Vec<i32> {
    let mut dial = start;
    let mut results = Vec::new();
    for &instruction in instructions {
        let mut total = instruction;
        let mut passes = 0;
        while total.abs() >= 100 {
            passes += 1;
            total -= total.signum() * 100;
        }
        let target = dial + total;
        if target > 99 || (dial != 0 && target <= 0) {
            passes += 1;
        }
        dial = rotate_dial(dial, instruction);
        results.push(passes);
    }
    results
}
/*
* day2, Part 1.
*/
pub fn calculate_part1(input: &str) -> usize {
    execute_instructions(50, &parse_input(input))
        .into_iter()
        .filter(|&x| x == 0)
        .count()
}

/*
 * day2, Part 2
 *
 */
pub fn calculate_part2(input: &str) -> usize {
        execute_instructions_part2(50, &parse_input(input))
        .into_iter()
        .sum::<i32>() as usize
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    #[test]
    fn test_parse() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(
            parse_input(&input),
            vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]
        );
    }

    #[test]
    fn test_execute_instructions() {
        assert_eq!(
            execute_instructions(50, &vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]),
            vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32]
        );
    }

    #[test]
    fn test_rotate_dial() {
        assert_eq!(rotate_dial(50, 15), 65);
        assert_eq!(rotate_dial(50, -15), 35);
        assert_eq!(rotate_dial(90, 15), 5);
        assert_eq!(rotate_dial(10, -15), 95);
    }


        #[test]
    fn test_execute_instructions_part2() {
        assert_eq!(
            execute_instructions_part2(50, &vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]),
            vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 1]
        );
        assert_eq!(
            execute_instructions_part2(50, &vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, 1000]),
            vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 10]
        );
        assert_eq!(
            execute_instructions_part2(50, &vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, 1098]),
            vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 11]
        );
        assert_eq!(
            execute_instructions_part2(0, &vec![-350]),
            vec![3]
        );
        assert_eq!(
            execute_instructions_part2(0, &vec![350]),
            vec![3]
        );
        assert_eq!(
            execute_instructions_part2(0, &vec![500]),
            vec![5]
        );
    }

    #[test]
    fn test_example_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(calculate_part1(&input), 3);
    }
    #[test]
    fn test_example_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(calculate_part2(&input), 6);
        assert_eq!(calculate_part2("L320"), 3);
        assert_eq!(calculate_part2("R320"), 3);
    }
}
