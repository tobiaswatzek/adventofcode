use std::collections::HashSet;
use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> usize {
    let (cells, guard_start) = parse_input(input);

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut guard_direction = GuardDirection::Up;
    let mut guard_pos = guard_start;

    while guard_pos.0 >= 0
        && guard_pos.0 < cells[0].len() as i64
        && guard_pos.1 >= 0
        && guard_pos.1 < cells.len() as i64
    {
        visited.insert(guard_pos);

        let next_pos = find_next_pos(guard_pos, guard_direction, &cells);

        match next_pos {
            None => break,
            Some((pos, dir)) => {
                guard_pos = pos;
                guard_direction = dir;
            }
        };
    }

    visited.len()
}

fn find_next_pos(
    current_pos: (i64, i64),
    current_direction: GuardDirection,
    map: &Vec<Vec<Cell>>,
) -> Option<((i64, i64), GuardDirection)> {
    let new_pos = match current_direction {
        GuardDirection::Up => (current_pos.0, current_pos.1 - 1),
        GuardDirection::Down => (current_pos.0, current_pos.1 + 1),
        GuardDirection::Left => (current_pos.0 - 1, current_pos.1),
        GuardDirection::Right => (current_pos.0 + 1, current_pos.1),
    };

    if new_pos.0 < 0
        || new_pos.0 as usize >= map[0].len()
        || new_pos.1 < 0
        || new_pos.1 as usize >= map.len()
    {
        return None;
    }

    match &map[new_pos.1 as usize][new_pos.0 as usize] {
        Cell::Empty => Some((new_pos, current_direction)),
        Cell::Obstacle => find_next_pos(
            current_pos,
            match current_direction {
                GuardDirection::Up => GuardDirection::Right,
                GuardDirection::Right => GuardDirection::Down,
                GuardDirection::Down => GuardDirection::Left,
                GuardDirection::Left => GuardDirection::Up,
            },
            map,
        ),
    }
}

#[derive(Clone, Copy)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

enum Cell {
    Empty,
    Obstacle,
}

fn parse_input(input: &str) -> (Vec<Vec<Cell>>, (i64, i64)) {
    let mut guard = None;
    for (y, line) in input.lines().filter(|l| !l.is_empty()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                guard = Some((x as i64, y as i64));
                break;
            }
        }
    }

    let cells = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' | '^' => Cell::Empty,
                    '#' => Cell::Obstacle,
                    _ => panic!("Unexpected char {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (cells, guard.unwrap())
}

fn solve_second(_input: &str) -> i64 {
    0
}
