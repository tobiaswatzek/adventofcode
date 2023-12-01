use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> i32 {
    let sum: i32 = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let first_number = l
                .chars()
                .find(|c| c.is_numeric())
                .expect("there must be a number");
            let last_number = l
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .expect("there must be a last number");

            let number = format!("{}{}", first_number, last_number)
                .parse::<i32>()
                .expect("must be a number");

            number
        })
        .sum();
    sum
}

fn solve_second(input: &str) -> i32 {
    let number_strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let sum: i32 = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let number_indices: Vec<(usize, String)> = l
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_numeric())
                .map(|(i, c)| (i, c.to_string()))
                .chain(number_strings.iter().enumerate().flat_map(|(i, e)| {
                    l.match_indices(e)
                        .map(move |(mi, _)| (mi, (i + 1).to_string()))
                }))
                .collect();

            let first = number_indices
                .iter()
                .min_by_key(|(i, _)| i)
                .and_then(|(_, n)| Some(n))
                .expect("there must be a first number");
            let last = number_indices
                .iter()
                .max_by_key(|(i, _)| i)
                .and_then(|(_, n)| Some(n))
                .expect("there must be a last number");

            let number = format!("{}{}", first, last)
                .parse::<i32>()
                .expect("must be a number");

            number
        })
        .sum();

    sum
}
