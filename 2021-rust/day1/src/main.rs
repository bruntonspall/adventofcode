use std::fs;
use day1::*;

fn main() {
    let numbers = parsenumbers(fs::read_to_string("day1.txt").expect("Couldn't read the day1.txt file"));
    let sum = count_increases(numbers);
    println!("{}", sum);
}
