/*
* day9, Part 1.
*/

type GeneratorResult = String;
type RunResult = u64;

// Use the max value for a 16 bit number as a symbol for free, so 0xFFFF is free
const FREE: u16 = u16::MAX;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input.to_string()
}

pub fn memory_from_dense_string(input: &str) -> Vec<u16> {
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

/// We find free blocks by starting at start and counting forwards until we find a FREE
fn find_next_free(input: &[u16], start: usize) -> usize {
    let mut start = start;
    while input[start] != FREE {
        start += 1;
    }
    start
}

/// We find blocks to move by starting at start and counting backwards until we find a block that isn't FREE
pub fn find_next_block(input: &[u16], start: usize) -> usize {
    let mut start = start;
    while input[start] == FREE {
        start -= 1;
    }
    start
}

pub fn defragment(input: &mut Vec<u16>) -> Vec<u16> {
    let mut next_block: usize = find_next_block(input, input.len() - 1);
    let mut next_free: usize = find_next_free(input, 0);
    while next_free < next_block {
        input[next_free] = input[next_block];
        input[next_block] = FREE;
        next_block = find_next_block(input, next_block - 1);
        next_free = find_next_free(input, next_free + 1);
    }
    input.clone()
}

pub fn checksum(input: &Vec<u16>) -> RunResult {
    input
        .iter()
        .enumerate()
        .map(|(i, id)| {
            if *id != FREE {
                (i as RunResult) * (*id as RunResult)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day9, part1, RunResult)]
pub fn part1(input: &GeneratorResult) -> RunResult {
    let mut memory = memory_from_dense_string(input);
    defragment(&mut memory);
    return checksum(&memory);
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
            memory_from_dense_string("12345"),
            vec![0, FREE, FREE, 1, 1, 1, FREE, FREE, FREE, FREE, 2, 2, 2, 2, 2]
        );
    }

    #[test]
    fn test_find_functions() {
        let memory = vec![0, 1, 2, FREE, 4, 5, FREE, FREE];
        assert_eq!(find_next_block(&memory, 7), 5);
        assert_eq!(find_next_block(&memory, 3), 2);
        assert_eq!(find_next_block(&memory, 2), 2);
        assert_eq!(find_next_free(&memory, 0), 3);
        assert_eq!(find_next_free(&memory, 4), 6);
        assert_eq!(find_next_free(&memory, 6), 6);
    }

    #[test]
    fn test_defrag() {
        let mut memory = vec![
            0, FREE, FREE, 1, 1, 1, FREE, FREE, FREE, FREE, 2, 2, 2, 2, 2,
        ];
        defragment(&mut memory);

        assert_eq!(
            memory,
            vec![0, 2, 2, 1, 1, 1, 2, 2, 2, FREE, FREE, FREE, FREE, FREE, FREE]
        );
    }

    #[test]
    fn test_complex_case() {
        let mut memory = memory_from_dense_string("2333133121414131402");
        assert_eq!(
            memory,
            vec![
                0, 0, FREE, FREE, FREE, 1, 1, 1, FREE, FREE, FREE, 2, FREE, FREE, FREE, 3, 3, 3,
                FREE, 4, 4, FREE, 5, 5, 5, 5, FREE, 6, 6, 6, 6, FREE, 7, 7, 7, FREE, 8, 8, 8, 8, 9,
                9
            ]
        );
        defragment(&mut memory);

        assert_eq!(
            memory,
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
                FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE
            ]
        );
    }

    #[test]
    fn test_boundary_cases() {
        let mut memory = vec![FREE, FREE, 1, 1, FREE, FREE, 2, 2, 2];
        defragment(&mut memory);
        assert_eq!(memory, vec![2, 2, 1, 1, 2, FREE, FREE, FREE, FREE]);
    }

    #[test]
    fn test_checksum() {
        let mut memory = memory_from_dense_string("2333133121414131402");
        defragment(&mut memory);
        assert_eq!(checksum(&memory), 1928);
        let mut memory = memory_from_dense_string("233313312141413140214");
        defragment(&mut memory);
        assert_eq!(checksum(&memory), 2837);
    }

    #[test]
    fn test_part1() {
        let input = "2333133121414131402";
        assert_eq!(part1(&input_generator(&input)), 1928);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(&input_generator(&input)), 0);
    }
}
