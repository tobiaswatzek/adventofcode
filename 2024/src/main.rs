use std::path::PathBuf;

use clap::Parser;

mod days;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(long)]
    day: u8,
    #[arg(long)]
    data_dir: PathBuf,
}

fn main() {
    let args = Args::parse();

    let (part_one, part_two) = days::solve_day(&args.day, &file_path(&args.data_dir, &args.day));

    println!(
        "Day {}:\n\tPart one: {part_one}\n\tPart two: {part_two}",
        args.day
    );
}

fn file_path(data_dir: &PathBuf, day: &u8) -> PathBuf {
    data_dir.join(format!("day{}.txt", day))
}