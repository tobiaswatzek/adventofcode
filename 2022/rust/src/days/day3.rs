use std::{collections::HashSet, fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("should have been able to read the file");
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

    let part_one = solve_part_one(&lines);
    let part_two = solve_part_two(&lines);

    (part_one.to_string(), part_two.to_string())
}

fn solve_part_one(lines: &Vec<&str>) -> u32 {
    lines.iter().fold(0, |acc, l| {
        let middle = l.len() / 2;
        let first: HashSet<char> = HashSet::from_iter(l[0..middle].chars());
        let second: HashSet<char> = HashSet::from_iter(l[middle..].chars());
        let in_both = first
            .intersection(&second)
            .nth(0)
            .expect("there has to be an intersection");

        acc + item_priority(in_both)
    })
}

fn solve_part_two(lines: &Vec<&str>) -> u32 {
    lines
        .as_slice()
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|s| {
                    let set: HashSet<char> = HashSet::from_iter(s.chars());
                    set
                })
                .reduce(|accum, item| HashSet::<char>::from_iter(accum.intersection(&item).map(|c| *c)))
                .and_then(|intersection| intersection.into_iter().nth(0))
                .and_then(|badge| Some(item_priority(&badge)))
                .expect("no badge could be found")
        })
        .sum()
}

fn item_priority(item: &char) -> u32 {
    match *item as u32 {
        i if i >= ('a' as u32) && i <= ('z' as u32) => i - 96,
        i => i - 38,
    }
}
