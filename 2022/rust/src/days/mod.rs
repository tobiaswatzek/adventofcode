use std::{collections::HashMap, path::PathBuf};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
pub mod day14;
mod day3;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day15;
mod day17;

pub fn solve_day(day: &u8, input_path: &PathBuf) -> (String, String) {
    let mut day_registry: HashMap<u8, fn(&PathBuf) -> (String, String)> = HashMap::new();
    day_registry.insert(1, day1::solve);
    day_registry.insert(3, day3::solve);
    day_registry.insert(5, day5::solve);
    day_registry.insert(6, day6::solve);
    day_registry.insert(7, day7::solve);
    day_registry.insert(8, day8::solve);
    day_registry.insert(9, day9::solve);
    day_registry.insert(10, day10::solve);
    day_registry.insert(11, day11::solve);
    day_registry.insert(12, day12::solve);
    day_registry.insert(13, day13::solve);
    day_registry.insert(14, day14::solve);
    day_registry.insert(15, day15::solve);
    day_registry.insert(17, day17::solve);

    let solve = day_registry
        .get(day)
        .expect("solution for given day is not implemented");

    solve(input_path)
}
