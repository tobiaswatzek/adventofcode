use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect();

    let first = solve_first(&sequences);
    let second = solve_second(&sequences);

    (first.to_string(), second.to_string())
}

fn solve_first(sequences: &Vec<Vec<i64>>) -> i64 {
    let result = sequences
        .iter()
        .map(|sequence| {
            let differences = find_null_sequence(sequence);
            let difference = differences
                .iter()
                .skip(1)
                .fold(0, |acc, d| d.last().unwrap() + acc);

            sequence.last().unwrap() + difference
        })
        .sum();

    result
}

fn solve_second(sequences: &Vec<Vec<i64>>) -> i64 {
    let result = sequences
        .iter()
        .map(|sequence| {
            let differences = find_null_sequence(sequence);
            let difference = differences
                .iter()
                .skip(1)
                .fold(0, |acc, d| d.first().unwrap() - acc);

            sequence.first().unwrap() - difference
        })
        .sum();

    result
}

fn find_null_sequence(sequence: &Vec<i64>) -> Vec<Vec<i64>> {
    if sequence.iter().all(|n| *n == 0) {
        return vec![];
    }

    let next_sequence: Vec<i64> = sequence
        .windows(2)
        .map(|pair| match pair {
            [a, b] => b - a,
            _ => unreachable!("window returns pairs"),
        })
        .collect();
    let mut result = find_null_sequence(&next_sequence);

    result.push(next_sequence);

    result
}
