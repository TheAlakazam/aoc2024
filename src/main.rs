use chrono::{Datelike, FixedOffset};
use clap::{Parser, Subcommand};
mod days;
mod etc;

use days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};
use etc::solution::Solution;
use reqwest::{
    header, Url,
};
use std::{fs, time::Instant};

pub type SolutionPair = (Solution, Solution);

const YEAR: u16 = 2024;

#[derive(Parser)]
#[command(name = "Advent of Code 2024")]
#[command(about = "Solutions for Advent of Code 2024", version = "1.0")]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Append)]
    days: Option<Vec<u8>>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run,
    GetInput,
}

fn main() {
    let cli = Cli::parse();
    let offset = FixedOffset::west_opt(5 * 60 * 60).unwrap();
    let today_date = chrono::Utc::now().with_timezone(&offset).day() as u8;
    let days: Vec<u8> = cli.days.unwrap_or_else(|| vec![today_date]);

    let mut runtime = 0.0;

    for day in days {
        match cli.command {
            Commands::Run => {
                let solver = get_day_solver(day);
                let start = Instant::now();
                let (part1, part2) = solver();
                let end = Instant::now();
                runtime += end.duration_since(start).as_secs_f64() * 1000.0;

                println!("Day {}:", day);
                println!("Part 1: {}", part1);
                println!("Part 2: {}", part2);
            }

            Commands::GetInput => {
                download_input(day as usize);
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
    // Read session cookie from .session file
    let session = fs::read_to_string(".session").expect("Could not find .session file");
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day)
        .parse::<Url>()
        .unwrap();

    let mut request_headers = header::HeaderMap::new();
    request_headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&format!("session={}", session.trim()))
            .expect("Invalid session header value"),
    );

    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(request_headers)
        .cookie_store(true)
        .build()
        .unwrap();
    let response = client.get(url).send().unwrap();

    if response.status().is_success() {
        let mut text = response.text().unwrap();
        // Remove trailing newline
        text.pop();
        let path = format!("./input/day{:02}.txt", day);
        fs::write(&path, text).unwrap();
        println!("Successfully downloaded input to {}", &path);
    } else {
        println!(
            "Failed to download input for day {}. Status: {}",
            day,
            response.status()
        );
        panic!(
            "Could not get input for day {}. Is your correct session cookie in your .session file?",
            day
        )
    }
}
