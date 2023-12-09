use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
};

use regex::Regex;

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let instructions = lines[0].chars();
    let map: HashMap<String, Paths> = lines[2..].iter().map(|l| parse_paths(l)).collect();

    const GOAL: &str = "ZZZ";
    let mut current = "AAA";
    let mut steps = vec![];
    for instruction in instructions.cycle() {
        if current == GOAL {
            break;
        }
        let paths = &map[current];
        current = match instruction {
            'L' => &paths.left,
            'R' => &paths.right,
            _ => unreachable!("unexpected instruction"),
        };
        steps.push(current);
    }

    steps.len()
}

fn solve_second(input: &str) -> u64 {
    0
}

fn parse_paths(line: &str) -> (String, Paths) {
    const KEY_GROUP: &str = "key";
    const LEFT_GROUP: &str = "left";
    const RIGHT_GROUP: &str = "right";
    let pattern = format!(
        r"(?P<{KEY_GROUP}>[A-Z]{{3}}) = \((?P<{LEFT_GROUP}>[A-Z]{{3}}), (?P<{RIGHT_GROUP}>[A-Z]{{3}})\)"
    );
    let re: Regex = Regex::new(pattern.as_str()).unwrap();
    let groups = re.captures(line).expect("regex must match");

    let key = &groups[KEY_GROUP];
    let left = &groups[LEFT_GROUP];
    let right = &groups[RIGHT_GROUP];

    (
        key.to_owned(),
        Paths {
            left: left.to_owned(),
            right: right.to_owned(),
        },
    )
}

#[derive(Debug)]
struct Paths {
    left: String,
    right: String,
}
