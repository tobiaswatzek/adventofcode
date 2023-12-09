mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::path::PathBuf;

use phf::phf_map;

type DayFn = fn(&PathBuf) -> (String, String);

static DAY_REGISTRY: phf::Map<u8, DayFn> = phf_map! {
    1u8 => day1::solve,
    2u8 => day2::solve,
    3u8 => day3::solve,
    4u8 => day4::solve,
    5u8 => day5::solve,
    6u8 => day6::solve,
    7u8 => day7::solve,
    8u8 => day8::solve,
    9u8 => day9::solve,
};

pub fn solve_day(day: &u8, input_path: &PathBuf) -> (String, String) {
    let solve = DAY_REGISTRY
        .get(day)
        .expect("solution for given day is not implemented");

    solve(input_path)
}
