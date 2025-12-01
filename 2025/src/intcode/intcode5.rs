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
    let program = parse_input(input);
    let mut cpu = IntCodeCPU::new_with_IO(program, vec![1]);
    cpu.run();
    println!("Output: {:?}", cpu.output);
    cpu.output.last().cloned().unwrap_or(0) as usize
}

/*
 * day2, Part 2
 *
 */
pub fn calculate_part2(_input: &str) -> usize {
    return 0    
}
