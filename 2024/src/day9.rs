/*
* day9, Part 1.
*/

type GeneratorResult = String;
type RunResult = usize;

// Use the max value for a 16 bit number as a symbol for free, so 0xFFFF is free
const FREE: u16 = u16::MAX;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input.to_string()
}

pub fn memory_from_dense_string(input: &String) -> Vec<u16> {
    let mut file: bool = true;
    let mut id: u16 = 0;
    let mut results: Vec<u16> = Vec::new();
    for c in input.chars() {
        let size = c.to_digit(10).unwrap() as u16;
        if file {
            for _ in 0..size {
                results.push(id);
            }
            id += 1;
            file = false;
        } else {
            for _ in 0..size {
                results.push(FREE);
            }
            file = true;
        }
    }
    results
}

pub fn defragment(input: &mut Vec<u16>) -> Vec<u16> {
    let next_block: usize = find_next_block(input, input.len() - 1);
    let next_free: usize = find_next_free(input, 0);
    input.clone()
}

/// We find free blocks by starting at start and counting forwards until we find a u16::MAX
fn find_next_free(input: &[u16], start: usize) -> usize {
    let mut start = start;
    while input[start] != u16::MAX {
        start += 1;
    }
    start
}

/// We find blocks to move by starting at start and counting backwards until we find a block that isn't u16::MAX
pub fn find_next_block(input: &[u16], start: usize) -> usize {
    let mut start = start;
    while input[start] == u16::MAX {
        start -= 1;
    }
    start
}

#[aoc(day9, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    todo!();
}

/*
 * day9, Part 2
 *
 */

#[aoc(day9, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::day9::*;

    #[test]
    fn test_generator() {
        assert_eq!(
            input_generator("2333133121414131402"),
            "2333133121414131402".to_string()
        );
    }

    #[test]
    fn test_memory_from_dense_string() {
        assert_eq!(
            memory_from_dense_string(&"12345".to_string()),
            vec![
                0,
                u16::MAX,
                u16::MAX,
                1,
                1,
                1,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                2,
                2,
                2,
                2,
                2
            ]
        );
    }

    #[test]
    fn test_find_functions() {
        let memory = vec![0, 1, 2, u16::MAX, 4, 5, u16::MAX, u16::MAX];
        assert_eq!(find_next_block(&memory, 7), 5);
        assert_eq!(find_next_block(&memory, 3), 2);
        assert_eq!(find_next_block(&memory, 2), 2);
        assert_eq!(find_next_free(&memory, 0), 3);
        assert_eq!(find_next_free(&memory, 4), 6);
        assert_eq!(find_next_free(&memory, 6), 6);
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
