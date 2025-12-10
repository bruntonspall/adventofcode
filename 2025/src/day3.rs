fn find_largest_pair(line: &Vec<u32>) -> u32 {
    line.iter()
        .enumerate()
        .map(
            |(pivot, first)| match line.split_at(pivot + 1).1.iter().max() {
                Some(value) => first * 10 + value,
                None => 0,
            },
        )
        .max()
        .unwrap()
}

pub fn calculate_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| find_largest_pair(&line.chars().map(|c| c.to_digit(10).unwrap()).collect()))
        .sum::<u32>() as usize
}

pub fn calculate_part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    #[test]
    fn test_find_largest_pair() {
        for (expected, input) in [
            (98, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            (89, vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            (78, vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            (92, vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
        ] {
            assert_eq!(find_largest_pair(&input), expected);
        }
    }
}
