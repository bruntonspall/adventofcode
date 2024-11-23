#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| { 
            l.parse::<u32>().unwrap()
        }).collect()
}


#[aoc(day1, part1, u32)]
pub fn part1(input: &Vec<u32>) -> u32 {
    input.windows(2).map(|i| if i[0] < i[1] { 1 } else { 0 }).sum()
}

#[aoc(day1, part2, u32)]
pub fn part2(input: &Vec<u32>) -> u32 {
    let windows: Vec<u32> = input.windows(3).map(|w|w.iter().sum()).collect();
    windows.windows(2).map(|i| if i[0] < i[1] { 1 } else { 0 }).sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    #[test]
    fn find_depths() {
        let input = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn compare_windows() {
        let input = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(part2(&input), 5);
    }
}