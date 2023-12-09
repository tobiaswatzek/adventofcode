use std::{
    collections::{HashMap, HashSet},
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
    let map: HashMap<String, Paths> = parse_paths(&lines[2..]);

    const GOAL: &str = "ZZZ";
    let mut current = "AAA";
    let mut steps = 0;
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
        steps += 1;
    }

    steps
}

fn solve_second(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let instructions = lines[0].chars();
    let map: HashMap<String, Paths> = parse_paths(&lines[2..]);

    let mut positions: Vec<&str> = map
        .keys()
        .filter_map(|k| {
            if k.ends_with('A') {
                Some(k.as_str())
            } else {
                None
            }
        })
        .collect();

    let possible_goals: HashSet<&str> = map
        .keys()
        .filter_map(|k| {
            if k.ends_with('Z') {
                Some(k.as_str())
            } else {
                None
            }
        })
        .collect();

    let mut steps = 0;
    for instruction in instructions.cycle() {
        if positions.iter().all(|pos| possible_goals.contains(pos)) {
            break;
        }

        for i in 0..positions.len() {
            let paths = &map[positions[i]];
            positions[i] = match instruction {
                'L' => &paths.left,
                'R' => &paths.right,
                _ => unreachable!("unexpected instruction"),
            };
        }

        steps += 1;
    }

    steps
}

fn parse_paths(lines: &[&str]) -> HashMap<String, Paths> {
    const KEY_GROUP: &str = "key";
    const LEFT_GROUP: &str = "left";
    const RIGHT_GROUP: &str = "right";
    let pattern = format!(
        r"(?P<{KEY_GROUP}>[\d\w]{{3}}) = \((?P<{LEFT_GROUP}>[\d\w]{{3}}), (?P<{RIGHT_GROUP}>[\d\w]{{3}})\)"
    );
    let re: Regex = Regex::new(pattern.as_str()).unwrap();

    lines
        .iter()
        .map(|line| {
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
        })
        .collect()
}

#[derive(Debug)]
struct Paths {
    left: String,
    right: String,
}
