use std::path::PathBuf;

use phf::phf_map;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

type DayFn = fn(&PathBuf) -> (String, String);

static DAY_REGISTRY: phf::Map<u8, DayFn> = phf_map! {
    1u8 => day01::solve,
    2u8 => day02::solve,
    3u8 => day03::solve,
    4u8 => day04::solve,
    5u8 => day05::solve,
};

pub fn solve_day(day: &u8, input_path: &PathBuf) -> (String, String) {
    let solve = DAY_REGISTRY
        .get(day)
        .expect("solution for given day is not implemented");

    solve(input_path)
}
