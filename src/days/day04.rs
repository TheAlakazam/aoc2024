use std::{fs, vec};

use clap::builder::Str;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = get_input();
    // let input = vec![
    //     "MMMSXXMASM".chars().collect(),
    //     "MSAMXMSMSA".chars().collect(),
    //     "AMXSXMAAMM".chars().collect(),
    //     "MSAMASMSMX".chars().collect(),
    //     "XMASAMXAMM".chars().collect(),
    //     "XXAMMXXAMA".chars().collect(),
    //     "SMSMSASXSS".chars().collect(),
    //     "SAXAMASAAA".chars().collect(),
    //     "MAMMMXMMMM".chars().collect(),
    //     "MXMXAXMASX".chars().collect(),
    // ];
    // Your solution here...
    let sol1: u64 = solve_part1(&input);
    let sol2: u64 = solve_part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part1(input: &[Vec<char>]) -> u64 {
    let word = "XMAS".to_string();
    find_word(input, word)
}

fn check_valid(m: i32, n: i32, (x, y): (i32, i32)) -> bool {
    x >= 0 && x < m && y >= 0 && y < n
}

fn solve_part2(input: &[Vec<char>]) -> u64 {
    let m = input.len();
    let n = input[0].len();
    let d1 = ((-1, -1), (1, 1));
    let d2 = ((-1, 1), (1, -1));

    let mut count = 0;

    for i in 0..m {
        for j in 0..n {
            if input[i][j] == 'A' {
                let a = (
                    (i as i32 + d1.0 .0, j as i32 + d1.0 .1),
                    (i as i32 + d1.1 .0, j as i32 + d1.1 .1),
                );
                let b = (
                    (i as i32 + d2.0 .0, j as i32 + d2.0 .1),
                    (i as i32 + d2.1 .0, j as i32 + d2.1 .1),
                );
                if !check_valid(m as i32, n as i32, a.0)
                    || !check_valid(m as i32, n as i32, a.1)
                    || !check_valid(m as i32, n as i32, b.0)
                    || !check_valid(m as i32, n as i32, b.1)
                {
                    continue;
                }
                let wd1: String = vec![
                    input[a.0 .0 as usize][a.0 .1 as usize],
                    input[i][j],
                    input[a.1 .0 as usize][a.1 .1 as usize],
                ]
                .into_iter()
                .collect();
                let wd2: String = vec![
                    input[b.0 .0 as usize][b.0 .1 as usize],
                    input[i][j],
                    input[b.1 .0 as usize][b.1 .1 as usize],
                ]
                .into_iter()
                .collect();
                if (wd1 == "MAS" || wd1 == "SAM") && (wd2 == "MAS" || wd2 == "SAM") {
                    count += 1;
                }
            }
        }
    }
    count
}

fn get_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string("input/day04.txt").unwrap();
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

pub fn find_word(grid: &[Vec<char>], word: String) -> u64 {
    let m = grid.len();
    let n = grid[0].len();

    let mut count = 0;
    let dirs: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..m {
        for j in 0..n {
            for dir in dirs.iter() {
                let mut x = i as i32;
                let mut y = j as i32;

                let mut t = String::new();
                for c in word.chars() {
                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        break;
                    }
                    if grid[x as usize][y as usize] != c {
                        break;
                    }
                    t.push(grid[x as usize][y as usize]);
                    x += dir.0;
                    y += dir.1;
                }
                count += (t == word) as u64;
                // if t == word {
                //     println!(
                //         "Start: ({}, {}) :: Dir: ({}, {}) :: Word: {} :: Count: {}",
                //         i, j, dir.0, dir.1, t, count
                //     );
                // }
            }
        }
    }
    count
}
