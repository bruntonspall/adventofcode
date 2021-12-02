pub fn parsenumbers(s: String) -> Vec<i32> {
    return s.lines().map(|a|a.trim().parse().expect("Expected a number")).collect();
}

pub fn count_increases(numbers: Vec<i32>) -> usize {
    return numbers.iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| b>a)
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let contents = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

        assert_eq!(vec![199,200,208,210,200,207,240,269,260,263], parsenumbers(contents.to_string()));
    }
    #[test]
    fn test_count_increases() {
        let contents = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

        assert_eq!(7, count_increases(parsenumbers(contents.to_string())));
    }
}