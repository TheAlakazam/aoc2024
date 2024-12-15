use itertools::Itertools;
use std::fs;
use std::{cmp::Ordering, collections::HashSet};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let (orderings, updates) = get_input();
    let sol1: u64 = solve_part1(&orderings, &updates);
    let sol2: u64 = solve_part2(&orderings, &updates);

    (Solution::from(sol1), Solution::from(sol2))
}

pub fn solve_part1(orderings: &HashSet<(i32, i32)>, updates: &Vec<Vec<i32>>) -> u64 {
    let mut count: u64 = 0;
    let compare = |x: &i32, y: &i32| !orderings.contains(&(*y, *x));

    for update in updates {
        if update.is_sorted_by(compare) {
            count += update[update.len() / 2] as u64;
        }
    }

    count
}

pub fn solve_part2(orderings: &HashSet<(i32, i32)>, updates: &Vec<Vec<i32>>) -> u64 {
    let compare = |x: &i32, y: &i32| {
        let (x, y) = (*x, *y);
        if orderings.contains(&(x, y)) {
            Ordering::Less
        } else if orderings.contains(&(y, x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };
    let mut updates = updates.clone();
    updates
        .iter_mut()
        .map(|update: &mut Vec<i32>| {
            if update.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
                0
            } else {
                update.sort_by(compare);
                update[update.len() / 2]
            }
        })
        .sum::<i32>() as u64
}

fn get_input() -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let input = fs::read_to_string("input/day05.txt").unwrap();
    let (orderings, updates) = input.split_once("\n\n").unwrap();
    let orderings: HashSet<(i32, i32)> = orderings
        .lines()
        .map(|line| {
            line.split("|")
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|line| line.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    (orderings, updates)
}
