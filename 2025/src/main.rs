use std::fs;
use clap::Parser;
mod intcode;

const DAYS: &[Day] = &[
    Day {
        year: 2019,
        day: 2,
        name: "Intcode 2019 Day 2",
        filename: "input/intcode/day02.txt",
        solve_part1: intcode::intcode2::calculate_part1,
        solve_part2: intcode::intcode2::calculate_part2,
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
                let part1 = (day.solve_part1)(&content);

                let part2 = (day.solve_part2)(&content);

                println!("  Part 1: {}", part1);
                println!("  Part 2: {}", part2);
            }
            Err(e) => {
                println!("  Error reading file: {}", e);
            }
        }

    }
    Ok(())
}