use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let mut calories_per_elf = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    calories_per_elf.sort_unstable();

    let max_calories = calories_per_elf.last().expect("There has to be an element");
    let sum_max_three: i32 = calories_per_elf.iter().rev().take(3).sum();

    (max_calories.to_string(), sum_max_three.to_string())
}
