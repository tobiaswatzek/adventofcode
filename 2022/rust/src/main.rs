use std::{env, error, path::PathBuf};

use adventofcode2022::days;

fn main() {
    let args = parse_args().expect("arguments are expected");

    let (part_one, part_two) = days::solve_day(&args.day, &args.file_path());

    println!(
        "Day {}:\n\tPart one: {part_one}\n\tPart two: {part_two}",
        args.day
    );
}

#[derive(Debug)]
struct Arguments {
    data_dir: PathBuf,
    day: u8,
}

impl Arguments {
    fn file_path(&self) -> PathBuf {
        self.data_dir.join(format!("day{}.txt", self.day))
    }
}

fn parse_args() -> Result<Arguments, Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let day = match args[1].parse::<u8>()? {
        d @ 1..=24 => d,
        d => return Err(format!("day must be between 1 and 24 but is {d}").into()),
    };

    let data_dir = match args[2].trim() {
        s if !s.is_empty() => PathBuf::from(s),
        _ => return Err("data dir must be passed as argument".into()),
    };

    Ok(Arguments { data_dir, day })
}
