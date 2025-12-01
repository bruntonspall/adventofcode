

use crate::intcode::intcodecpu::IntCodeCPU;

fn parse_input(_input: &str) -> Vec<i32> {
    _input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect()
}

/*
* day2, Part 1.
*/
pub fn calculate_part1(input: &str) -> usize {
    let mut program = parse_input(input);
    program[1] = 12;
    program[2] = 2;
    let mut cpu = IntCodeCPU::new(program);
    cpu.run();
    cpu.memory[0] as usize
}

/*
 * day2, Part 2
 *
 */
pub fn calculate_part2(_input: &str) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = parse_input(_input);
            program[1] = noun;
            program[2] = verb;
            let mut cpu = IntCodeCPU::new(program);
            cpu.run();
            if cpu.memory[0] == 19690720 {
                return (noun * 100 + verb) as usize;
            }
        }
    }
    return 0    
}


#[cfg(test)]
mod tests {
    use crate::intcode::intcode2::*;

    #[test]
    fn test_generator() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        assert_eq!(parse_input(&input), vec![1,9,10,3,2,3,11,0,99,30,40,50]);
    }

    #[test]
    fn test_example_part1() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let program = parse_input(input);
        let mut cpu = IntCodeCPU::new(program);
        cpu.run();
        assert_eq!(cpu.memory[0] as usize, 3500);
    }
}
