pub fn parsenumbers(s: String) -> Vec<i32> {
    return s.lines().map(|a|a.trim().parse().expect("Expected a number")).collect();
}

pub fn count_increases(numbers: Vec<i32>) -> usize {
    return numbers.iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| b>a)
        .count();
}

pub fn create_sliding_window(numbers: Vec<i32>) -> Vec<i32> {
    let start = numbers.iter().take(3).sum();
    let mut v = Vec::from([start]);
    v.extend(numbers.iter().zip(numbers.iter().skip(3)).scan(start, |state, (&first,&last)| {
        *state = *state - first + last;
        Some(*state)
    }));
    v
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
    #[test]
    fn test_create_sliding_window() {
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

        assert_eq!(vec![607, 618, 618, 617, 647, 716, 769, 792], create_sliding_window(parsenumbers(contents.to_string())));
    }

}