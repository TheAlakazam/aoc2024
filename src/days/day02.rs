use std::{
    fs::{self, read},
    path,
};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = get_input();
    let sol1: u64 = solve_part1(&input);
    let sol2: u64 = solve_part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn get_input() -> Vec<Vec<u32>> {
    let lines = String::from_utf8(read("input/day02.txt").unwrap()).unwrap();
    lines
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve_part1(input: &Vec<Vec<u32>>) -> u64 {
    input.iter().filter(|x| check_if_safe(x)).count() as u64
}

fn solve_part2(input: &Vec<Vec<u32>>) -> u64 {
    input
        .iter()
        .map(|nums| {
            if check_if_safe(&nums) {
                return 1;
            }

            for to_drop in 0..nums.len() {
                let mut new_nums = nums.clone();
                new_nums.remove(to_drop);
                if check_if_safe(&&new_nums) {
                    return 1;
                }
            }
            0
        })
        .sum()
}

fn check_if_safe(input: &&Vec<u32>) -> bool {
    let direction = input[0] < input[1];
    let is_safe = input
        .iter()
        .zip(input.iter().skip(1))
        .fold(true, |valid, (a, b)| {
            let curr_direction = a < b && direction || a > b && !direction;
            let dist = a.abs_diff(*b);
            let step_dist = (1..=3).contains(&dist);
            curr_direction && step_dist && valid
        });
    is_safe
}
