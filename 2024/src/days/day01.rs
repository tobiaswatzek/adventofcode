use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> i64 {
    let mut columns =
        input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .fold((vec![], vec![]), |mut acc, l| {
                let nums: Vec<i64> = l
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().expect("This must be a number"))
                    .collect();

                acc.0.push(nums[0]);
                acc.1.push(nums[1]);

                acc
            });

    columns.0.sort();
    columns.1.sort();

    let sum: i64 = columns
        .0
        .iter()
        .zip(columns.1.iter())
        .fold(0, |acc, nums| acc + (nums.0 - nums.1).abs());

    sum
}

fn solve_second(input: &str) -> usize {
    let columns =
        input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .fold((vec![], vec![]), |mut acc, l| {
                let nums: Vec<_> = l
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().expect("This must be a number"))
                    .collect();

                acc.0.push(nums[0]);
                acc.1.push(nums[1]);

                acc
            });

    columns
        .0
        .iter()
        .map(|n| n * columns.1.iter().filter(|n2| n2 == &n).count())
        .sum()
}
