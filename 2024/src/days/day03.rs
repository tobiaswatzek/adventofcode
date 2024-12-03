use std::{fs, path::PathBuf};

use regex::Regex;

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> i64 {
    parse_mul(input)
}

fn parse_mul(input: &str) -> i64 {
    const FIRST_NUM: &str = "first";
    const SECOND_NUM: &str = "second";
    let pattern = format!(r"mul\((?P<{FIRST_NUM}>\d{{1,3}}),(?P<{SECOND_NUM}>\d{{1,3}})\)");
    let re = Regex::new(pattern.as_str()).unwrap();

    re.captures_iter(input)
        .map(|m| {
            let x = m[FIRST_NUM].parse::<i64>().unwrap();
            let y = m[SECOND_NUM].parse::<i64>().unwrap();

            x * y
        })
        .sum()
}

fn solve_second(input: &str) -> i64 {
    let enabled_re = Regex::new(
        r"((?:^|do\(\))[\s\S]*?(?:mul\((?:\d{1,3}),(?:\d{1,3})\))+[\s\S]*?(?:don't\(\)|$))+",
    )
    .unwrap();

    enabled_re
        .captures_iter(input)
        .map(|g| {
            let enabled_part = &g[0];
            parse_mul(enabled_part)
        })
        .sum()
}
