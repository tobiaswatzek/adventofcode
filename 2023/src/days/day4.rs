use std::{
    collections::HashSet,
    fs,
    path::PathBuf,
};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> i32 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let cards = l[8..]
                .split('|')
                .map(|nums| {
                    nums.split(' ')
                        .filter_map(|n| n.trim().parse::<i32>().ok())
                        .collect::<HashSet<i32>>()
                })
                .collect::<Vec<_>>();
            let (winning_numbers, scratched_numbers) = match &cards[..] {
                [first, second] => (first, second),
                _ => unreachable!(),
            };

            let count: u32 = winning_numbers
                .intersection(&scratched_numbers)
                .count()
                .try_into()
                .unwrap();

            if count == 0 {
                return 0;
            }

            let points: i32 = 2_i32.pow(count - 1);

            points
        })
        .sum()
}

fn solve_second(_input: &str) -> i32 {
    0
}
