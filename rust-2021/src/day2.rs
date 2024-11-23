
#[derive(PartialEq, Debug)]
pub enum Command {
    Forward (i32),
    Up (i32),
    Down (i32),
}

pub fn parse_line(input: &str) -> Command {
    let mut args = input.trim().split(" ");
    let command = args.next();
    let arg = args.next().unwrap_or("0");
    match command {
        Some("forward") => Command::Forward(arg.parse::<i32>().unwrap()),
        Some("up") => Command::Up(arg.parse::<i32>().unwrap()),
        Some("down") => Command::Down(arg.parse::<i32>().unwrap()),
        _ => panic!("This shouldn't happen")
    }

}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(parse_line).collect()
}


#[aoc(day2, part1, i32)]
pub fn part1(input: &Vec<Command>) -> i32 {
    use Command::*;

    let total = input.iter().fold((0,0), |acc, command| match command {
        Forward(x) => (acc.0 + x,acc.1),
        Up(x) => (acc.0, acc.1 - x),
        Down(x) => (acc.0, acc.1 + x)
    });
    total.0 * total.1
}

#[aoc(day2, part2, i32)]
pub fn part2(input: &Vec<Command>) -> i32 {
    use Command::*;

    let total = input.iter().fold((0,0,0), |acc, command| match command {
        Forward(x) => (acc.0 + x,acc.1+x*acc.2, acc.2),
        Up(x) => (acc.0, acc.1, acc.2-x),
        Down(x) => (acc.0, acc.1, acc.2+x)
    });
    total.0 * total.1
}

#[cfg(test)]
mod tests {
    use crate::day2::{parse_line, part1, part2};
    use crate::day2::Command::*;

    #[test]
    fn test_parse() {
        assert_eq!(Forward(6), parse_line("forward 6"));
        assert_eq!(Down(2), parse_line("down 2"));
        assert_eq!(Up(77), parse_line("up 77"));
    }

    #[test]
    fn find_depths() {
        let input = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn use_aim() {
        let input = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(part2(&input), 900);
    }
}