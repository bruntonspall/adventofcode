
    fn _parse_input(_input: &str) -> Vec<usize> {
        unimplemented!()
    }

/*
* day2, Part 1.
*/
    pub fn calculate_part1(_input: &str) -> usize {
        0
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
    use crate::intcode::intcode2::*;

    #[test]
    fn test_generator() {}

    #[test]
    fn test_part1() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        assert_eq!(calculate_part1(&input), 30);
    }

    #[test]
    fn test_part2() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        assert_eq!(calculate_part2(&input), 0);
    }
}
