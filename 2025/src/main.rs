use std::fs;
use clap::Parser;
mod intcode;
use std::time::Instant;
mod day1;


const DAYS: &[Day] = &[
    Day {
        year: 2019,
        day: 2,
        name: "Intcode 2019 Day 2",
        filename: "input/intcode/day02.txt",
        solve_part1: intcode::intcode2::calculate_part1,
        solve_part2: intcode::intcode2::calculate_part2,
    },
    Day {
        year: 2019,
        day: 5,
        name: "Intcode 2019 Day 5",
        filename: "input/intcode/day05.txt",
        solve_part1: intcode::intcode5::calculate_part1,
        solve_part2: intcode::intcode5::calculate_part2,
    },
    Day {
        year: 2025,
        day: 1,
        name: "2025 Day 1",
        filename: "input/2025/day01.txt",
        solve_part1: day1::calculate_part1,
        solve_part2: day1::calculate_part2,
    },
    // Add more days here as needed
];
struct Day {
    year: u16,
    day: u8,
    name: &'static str,
    filename: &'static str,
    solve_part1: fn(&str) -> usize,
    solve_part2: fn(&str) -> usize,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: Option<u16>,
    #[arg(short, long)]
    day: Option<u8>,
}
fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();
    if args.year.is_none() && args.day.is_none() {
        for day in DAYS {
            println!("{} — {} is {}", day.year, day.day, day.name);
        }
        std::process::exit(1);
    }
    let total_start = Instant::now();

    for day in DAYS {

        match args.year {
            Some(y) if y != day.year => continue,
            _ => {}
        }
        match args.day {
            Some(d) if d != day.day => continue,
            _ => {}
        }
        println!("Day {} — {}", day.name, day.filename);
        match fs::read_to_string(&day.filename) {
            Ok(content) => {
                let before = Instant::now();
                let part1 = (day.solve_part1)(&content);
                let p1_dur = before.elapsed();

                let before = Instant::now();
                let part2 = (day.solve_part2)(&content);
                let p2_dur = before.elapsed();

                println!("  Part 1: {} in {} ms", part1, p1_dur.as_millis());
                println!("  Part 2: {} in {} ms", part2, p2_dur.as_millis());
            }
            Err(e) => {
                println!("  Error reading file: {}", e);
            }
        }

    }
    println!("Total time: {} ms", total_start.elapsed().as_millis());
    Ok(())
}