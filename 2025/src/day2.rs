pub fn parse_id_range(id_range: &str) -> (u64, u64) {
    let mut ids = id_range
        .split("-")
        .map(|id| id.parse::<u64>().expect("Expected a whole number"));
    (ids.next().unwrap(), ids.next().unwrap())
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input.split(",").map(parse_id_range).collect()
}

pub fn has_repeated_sequence_twice(input: &str) -> bool {
    let (a, b) = input.split_at(input.len() / 2);
    return a == b;
}

pub fn generate_id_sequence(start: u64, end: u64) -> Vec<String> {
    // let mut result = Vec::new();
    // for i in start..end+1 {
    //     result.push(i.to_string())
    // }
    // result
    (start..end + 1).map(|digit| digit.to_string()).collect()
}

pub fn find_invalid_ids(ids: Vec<(u64, u64)>) -> Vec<u64> {
    ids.iter()
        .map(|(start, end)| generate_id_sequence(*start, *end))
        .map(|id_sequence| {
            id_sequence
                .iter()
                .filter(|id| has_repeated_sequence_twice(&id))
                .map(|id| id.parse::<u64>().expect("Expected integer ids"))
                .collect::<Vec<u64>>()
        })
        .flatten()
        .collect()
}

pub fn calculate_part1(input: &str) -> usize {
    let ids = parse_input(input);
    find_invalid_ids(ids).into_iter().sum::<u64>() as usize
}

pub fn calculate_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    #[test]
    fn test_has_repeated_sequence_twice() {
        for (expected, input) in [
            (true, "11"),
            (false, "13"),
            (false, "15"),
            (true, "22"),
            (false, "98"),
            (true, "99"),
            (false, "100"),
            (true, "1188511885"),
        ] {
            assert_eq!(has_repeated_sequence_twice(input), expected);
        }
    }

    #[test]
    fn test_generate_id_sequence() {
        for (expected, start, end) in [
            (
                vec![
                    "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22",
                ],
                11,
                22,
            ),
            (
                vec![
                    "95", "96", "97", "98", "99", "100", "101", "102", "103", "104", "105",
                ],
                95,
                105,
            ),
        ] {
            assert_eq!(generate_id_sequence(start, end), expected);
        }
    }

    #[test]
    fn test_parse_id_range() {
        for (expected, id_range) in [
            ((11, 22), "11-22"),
            ((95, 105), "95-105"),
            ((38593856, 38593862), "38593856-38593862"),
            ((1188511880, 1188511890), "1188511880-1188511890"),
            ((7777695646, 7777817695), "7777695646-7777817695"),
        ] {
            assert_eq!(parse_id_range(id_range), expected);
        }
    }

    #[test]
    fn test_parse_input() {
        for (expected, input) in [(
            vec![(11, 22), (95, 115), (998, 1012), (1188511880, 1188511890), (38593856,38593862),(979959461,980003045)],
            "11-22,95-115,998-1012,1188511880-1188511890,38593856-38593862,979959461-980003045",
        )] {
            assert_eq!(parse_input(input), expected);
        }
    }

    #[test]
    fn test_find_invalid_ids() {
        for (expected, input) in [(
            vec![11, 22, 99, 1010, 1188511885],
            vec![(11, 22), (95, 115), (998, 1012), (1188511880, 1188511890)],
        )] {
            assert_eq!(find_invalid_ids(input), expected);
        }
    }

    #[test]
    fn test_calculate_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(calculate_part1(input), 1227775554);
    }
}
