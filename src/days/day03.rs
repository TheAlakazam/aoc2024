use std::fs;

use crate::{Solution, SolutionPair};
use regex::Regex;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = get_input();
    let sol1: u64 = solve_part1(&input) as u64;
    let sol2: u64 = solve_part2(&input) as u64;

    (Solution::from(sol1), Solution::from(sol2))
}

fn get_input() -> String {
    fs::read_to_string("input/day03.txt")
        .expect("Unable to read file")
        .trim()
        .to_string()
}

fn solve_part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    // println!("{} {}", re, input);
    re.captures_iter(input)
        .map(|c| c.extract())
        .fold(0, |sum, (_, [a, b])| {
            sum + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
}

fn solve_part2(input: &str) -> i32 {
    let segments = Regex::new(r"(?s)(?:^|do\(\))(.*?)(?:don't\(\)|$)").unwrap();
    segments
        .captures_iter(input)
        .map(|c| c.extract())
        .fold(0, |sum, (_, [seg])| sum + solve_part1(seg))
}
