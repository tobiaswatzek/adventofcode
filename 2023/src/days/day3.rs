use std::{collections::HashSet, fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let grid: Vec<Vec<char>> = parse(&input);
    let first = solve_first(&grid);
    let second = solve_second(&grid);
    (first.to_string(), second.to_string())
}

fn solve_first(grid: &Vec<Vec<char>>) -> i32 {
    let mut part_numbers = HashSet::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '.' || c.is_numeric() {
                continue;
            }

            find_part_numbers(x, y, &grid, &mut part_numbers);
        }
    }

    part_numbers.iter().map(|(_, _, pn)| pn).sum()
}

fn solve_second(grid: &Vec<Vec<char>>) -> i32 {
    let mut gear_ratios = vec![];
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c != '*' {
                continue;
            }
            let mut part_numbers = HashSet::new();

            find_part_numbers(x, y, grid, &mut part_numbers);

            match part_numbers.iter().collect::<Vec<_>>()[..] {
                [(_, _, a), (_, _, b)] => {
                    gear_ratios.push(a * b);
                }
                _ => (),
            }
        }
    }

    gear_ratios.iter().sum()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

fn insert_some(item: Option<(usize, usize, i32)>, set: &mut HashSet<(usize, usize, i32)>) {
    match item {
        Some(num) => {
            set.insert(num);
        }
        _ => (),
    }
}

fn find_part_number(x: usize, y: usize, grid: &Vec<Vec<char>>) -> Option<(usize, usize, i32)> {
    let line_len = grid.first().unwrap().len();
    if x > line_len || y > grid.len() {
        return None;
    }

    if !grid[y][x].is_numeric() {
        return None;
    }

    let start_idx = grid[y][..x]
        .iter()
        .enumerate()
        .rfind(|(_, c)| !c.is_numeric())
        .and_then(|(i, _)| Some(i + 1))
        .unwrap_or(0);
    let end_offset = grid[y][x..]
        .iter()
        .enumerate()
        .find(|(_, c)| !c.is_numeric())
        .and_then(|(i, _)| Some(i))
        .unwrap_or(line_len - x);

    let num_str = grid[y][start_idx..x + end_offset]
        .iter()
        .collect::<String>();

    let num = num_str.parse().expect("must be a number");

    Some((start_idx, y, num))
}

fn find_part_numbers(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    part_numbers: &mut HashSet<(usize, usize, i32)>,
) {
    let top_left = find_part_number(x - 1, y - 1, &grid);
    insert_some(top_left, part_numbers);
    let top = find_part_number(x, y - 1, &grid);
    insert_some(top, part_numbers);
    let top_right = find_part_number(x + 1, y - 1, &grid);
    insert_some(top_right, part_numbers);
    let left = find_part_number(x - 1, y, &grid);
    insert_some(left, part_numbers);
    let right = find_part_number(x + 1, y, &grid);
    insert_some(right, part_numbers);
    let bottom_left = find_part_number(x - 1, y + 1, &grid);
    insert_some(bottom_left, part_numbers);
    let bottom = find_part_number(x, y + 1, &grid);
    insert_some(bottom, part_numbers);
    let bottom_right = find_part_number(x + 1, y + 1, &grid);
    insert_some(bottom_right, part_numbers);
}
