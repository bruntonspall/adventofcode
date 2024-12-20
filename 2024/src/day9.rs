/*
* day9, Part 1.
*/

use std::usize::MAX;

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

/// We find a block by finding the first block with the number, and the size by continueing until the block isn't that id
fn find_block_by_id(input: &[u16], id: u16) -> (usize, usize) {
    let mut start = 0;
    let mut size: usize = 0;
    while input[start] != id {
        start += 1;
    }
    while start + size < input.len() && input[start + size] == id {
        size += 1;
    }
    (start, size)
}

/// We look for free blocks of at least the right size that are before end
pub fn find_first_free_of_size(input: &[u16], size: usize, end: usize) -> usize {
    let mut start = 0;
    let mut found = 0;
    while found < size && start < end {
        while input[start] != FREE {
            start += 1;
        }
        while input[start + found] == FREE && start < end {
            found += 1;
            if found == size {
                return start;
            }
        }
        found = 0;
        start += 1;
    }
    return MAX;
}

pub fn defragment_without_splits(input: &mut Vec<u16>) -> Vec<u16> {
    // First we find the last ID in the disk
    let mut i = input.len() - 1;
    while input[i] == FREE {
        i -= 1
    }
    let mut id = input[i];
    // Now we go through them all
    while id > 0 {
        let (block, size) = find_block_by_id(input, id);
        let free = find_first_free_of_size(input, size, block);
        if free != MAX {
            for j in 0..size {
                input[free + j] = id;
                input[block + j] = FREE;
            }
        }
        id -= 1;
    }
    input.clone()
}

#[aoc(day9, part2, RunResult)]
pub fn part2(input: &GeneratorResult) -> RunResult {
    let mut memory = memory_from_dense_string(input);
    defragment_without_splits(&mut memory);
    return checksum(&memory);
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
    fn test_find_size_functions() {
        let memory = vec![
            //0 1 2  3     4     5  6  7     8  9  10 11 12    13    14    15 16 17    18    19
            0, 1, 1, FREE, FREE, 2, 2, FREE, 4, 4, 4, 4, FREE, FREE, FREE, 5, 5, FREE, FREE, FREE,
            FREE,
        ];
        assert_eq!(find_block_by_id(&memory, 5), (15, 2));
        assert_eq!(find_block_by_id(&memory, 4), (8, 4));
        assert_eq!(find_block_by_id(&memory, 2), (5, 2));
        assert_eq!(find_block_by_id(&memory, 1), (1, 2));
        assert_eq!(find_first_free_of_size(&memory, 1, 20), 3);
        assert_eq!(find_first_free_of_size(&memory, 2, 20), 3);
        assert_eq!(find_first_free_of_size(&memory, 3, 20), 12);
        assert_eq!(find_first_free_of_size(&memory, 4, 20), 17);
        // Test out of bounds
        assert_eq!(find_first_free_of_size(&memory, 4, 12), MAX);
    }

    #[test]
    fn test_block_defrag() {
        // This original one shouldn't change at all
        let mut memory = vec![
            0, FREE, FREE, 1, 1, 1, FREE, FREE, FREE, FREE, 2, 2, 2, 2, 2,
        ];
        defragment_without_splits(&mut memory);

        assert_eq!(
            memory,
            vec![0, FREE, FREE, 1, 1, 1, FREE, FREE, FREE, FREE, 2, 2, 2, 2, 2,]
        );

        // Let's try a new one
        let mut memory = vec![
            0, FREE, FREE, 1, 1, 1, FREE, FREE, FREE, FREE, 2, 2, FREE, FREE, 3, 3, 3,
        ];
        defragment_without_splits(&mut memory);

        assert_eq!(
            memory,
            vec![0, 2, 2, 1, 1, 1, 3, 3, 3, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE]
        );
    }

    #[test]
    fn test_part2() {
        let input = "2333133121414131402";
        assert_eq!(part2(&input_generator(&input)), 2858);
    }
}
