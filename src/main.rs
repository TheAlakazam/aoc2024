use aoc2024::AocClient;
use chrono::{Datelike, FixedOffset};
use clap::{Parser, Subcommand};
mod days;
mod etc;

use days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};
use etc::solution::Solution;
use std::{fs, time::Instant};

pub type SolutionPair = (Solution, Solution);

const YEAR: u32 = 2024;
const BASE_URL: &str = "https://adventofcode.com";

#[derive(Parser)]
#[command(name = "Advent of Code 2024")]
#[command(about = "Solutions for Advent of Code 2024", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long, action = clap::ArgAction::Append)]
        days: Option<Vec<u8>>,
    },
    GetInput {
        #[arg(short, long, action = clap::ArgAction::Append)]
        days: Option<Vec<u8>>,
    },
    Submit {
        #[arg(short, long, action = clap::ArgAction::Append)]
        days: Option<Vec<u8>>,
        #[arg(short, long, action = clap::ArgAction::Set)]
        level: Option<u8>,
    },
}

fn main() {
    let cli = Cli::parse();
    let offset = FixedOffset::west_opt(5 * 60 * 60).unwrap();
    let today_date = chrono::Utc::now().with_timezone(&offset).day() as u8;

    let mut runtime = 0.0;

    match cli.command {
        Commands::Run { days } => {
            let days: Vec<u8> = days.unwrap_or_else(|| vec![today_date]);
            for day in days {
                let solver = get_day_solver(day);
                let start = Instant::now();
                let (part1, part2) = solver();
                let end = Instant::now();
                runtime += end.duration_since(start).as_secs_f64() * 1000.0;

                println!("Day {}:", day);
                println!("Part 1: {}", part1);
                println!("Part 2: {}", part2);
            }
        }
        Commands::GetInput { days } => {
            let days: Vec<u8> = days.unwrap_or_else(|| vec![today_date]);
            for day in days {
                download_input(day as usize);
            }
        }
        Commands::Submit { days, level } => {
            let days: Vec<u8> = days.unwrap_or_else(|| vec![today_date]);
            for day in days {
                let start = Instant::now();

                let solver = get_day_solver(day);
                let (part1, part2) = solver();
                match level {
                    Some(1) => {
                        println!("Day {}: {}", day, part1);
                        upload_solution(day as usize, level.unwrap(), &part1.to_string());
                    }
                    Some(2) => {
                        println!("Day {}: {}", day, part2);
                        upload_solution(day as usize, level.unwrap(), &part2.to_string());
                    }
                    None => {
                        println!("Day {}:", day);
                        println!("Part 1: {}", part1);
                        upload_solution(day as usize, level.unwrap(), &part1.to_string());

                        println!("Part 2: {}", part2);
                        upload_solution(day as usize, level.unwrap(), &part2.to_string());
                    }
                    _ => panic!("Invalid level"),
                }
                let end = Instant::now();
                runtime += end.duration_since(start).as_secs_f64() * 1000.0;
            }
        }
    }

    println!("Total runtime: {:.4} ms", runtime);
}

fn get_day_solver(day: u8) -> fn() -> SolutionPair {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        9 => day09::solve,
        10 => day10::solve,
        11 => day11::solve,
        12 => day12::solve,
        13 => day13::solve,
        14 => day14::solve,
        15 => day15::solve,
        16 => day16::solve,
        17 => day17::solve,
        18 => day18::solve,
        19 => day19::solve,
        20 => day20::solve,
        21 => day21::solve,
        22 => day22::solve,
        23 => day23::solve,
        24 => day24::solve,
        25 => day25::solve,
        _ => unimplemented!(),
    }
}

fn download_input(day: usize) {
    let client = AocClient::new(BASE_URL, &get_session());
    client
        .download_input(
            YEAR,
            day as u32,
            format!("input/day{:02}.txt", day).as_str().as_ref(),
        )
        .expect("Failed to download input");
}

fn upload_solution(day: usize, level: u8, answer: &str) {
    let client = AocClient::new(BASE_URL, &get_session());
    let res = client
        .upload_solution(YEAR, day as u32, level as u32, answer)
        .expect("Failed to upload solution");
    if res.contains("That's the right answer") {
        println!("✅ That's the right answer!");
    } else {
        println!("❌ That's not the right answer.")
    }
}

fn get_session() -> String {
    fs::read_to_string(".session").expect("Failed to read session.txt")
}
