mod day1;
mod day2;

use std::path::PathBuf;

use phf::phf_map;

type DayFn = fn(&PathBuf) -> (String, String);

static DAY_REGISTRY: phf::Map<u8, DayFn> = phf_map! {
    1u8 => day1::solve,
    2u8 => day2::solve,
};

pub fn solve_day(day: &u8, input_path: &PathBuf) -> (String, String) {
    let solve = DAY_REGISTRY
        .get(day)
        .expect("solution for given day is not implemented");

    solve(input_path)
}
