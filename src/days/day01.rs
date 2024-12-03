use std::{fs::read, iter::zip};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let (a, b): (Vec<u32>, Vec<u32>) = read_input();

    // Your solution here...
    let sol1: u64 = solve_part1(&mut a.clone(), &mut b.clone()) as u64;
    let sol2: u64 = solve_part2(&a, &b) as u64;
    (Solution::from(sol1), Solution::from(sol2))
}

pub fn read_input() -> (Vec<u32>, Vec<u32>) {
    let mut first = Vec::new();
    let mut second = Vec::new();
    // Your solution here...
    let lines = String::from_utf8(read("input/day01.txt").unwrap()).unwrap();
    lines.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        if let (Some(first_part), Some(second_part)) = (parts.next(), parts.next()) {
            first.push(first_part.parse().unwrap());
            second.push(second_part.parse().unwrap());
        }
    });
    (first, second)
}

fn solve_part1(a: &mut Vec<u32>, b: &mut Vec<u32>) -> u32 {
    a.sort();
    b.sort();

    zip(a, b).fold(0, |acc, (x, y)| acc + (*x).abs_diff(*y))
}

fn solve_part2(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    let mut b_cmap = std::collections::HashMap::new();
    for &num in b {
        *b_cmap.entry(num).or_insert(0) += 1;
    }
    a.iter().fold(0, |acc, &num| {
        let bcount = b_cmap.get(&num).unwrap_or(&0);
        acc + (bcount * num)
    }) as u32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let mut a = vec![1, 3, 5, 7];
        let mut b = vec![2, 4, 6, 8];
        assert_eq!(solve_part1(&mut a, &mut b), 4);

        let mut a = vec![10, 20, 30];
        let mut b = vec![15, 25, 35];
        assert_eq!(solve_part1(&mut a, &mut b), 15);
    }

    #[test]
    fn test_solve_part2() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(solve_part2(&a, &b), 31);
    }
}
