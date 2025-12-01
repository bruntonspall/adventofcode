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
    (start + turn + 100) % 100
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
pub fn calculate_part2(_input: &str) -> usize {
    0
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
            execute_instructions(50, &parse_input(&input)),
            vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32]
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
}
