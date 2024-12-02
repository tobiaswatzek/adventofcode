use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter(|l| {
            let levels: Vec<_> = l
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("Levels must be numbers"))
                .collect();

            is_safe(&levels)
        })
        .count()
}

fn is_safe(levels: &Vec<i64>) -> bool {
    let all_decreasing = levels.windows(2).all(|w| {
        let diff = w[0] - w[1];
        diff > 0 && diff <= 3
    });
    let all_increasing = levels.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff > 0 && diff <= 3
    });

    all_decreasing || all_increasing
}

fn solve_second(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter(|l| {
            let levels: Vec<_> = l
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("Levels must be numbers"))
                .collect();

            if is_safe(&levels) {
                return true;
            }

            for (i, _) in levels.iter().enumerate() {
                let damped_levels = levels
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, l)| if idx == i { None } else { Some(*l) })
                    .collect();
                if is_safe(&damped_levels) {
                    return true;
                }
            }

            false
        })
        .count()
}
