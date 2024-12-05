use regex::Regex;
use std::fmt::Debug;
use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}


fn solve_first(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let horizontal: usize = lines
        .iter()
        .map(|line| count_search(line))
        .sum();

    let lines_chars: Vec<_> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let rotated = rotate_90deg(&lines_chars);

    let vertical: usize = rotated
        .iter()
        .map(|line| count_search(&line.iter().collect::<String>()))
        .sum();

    let diagonal_1: usize = get_diagonals(&lines_chars)
        .iter()
        .map(|line| count_search(&line.iter().collect::<String>()))
        .sum();

    let diagonal_2: usize = get_diagonals(&rotated)
        .iter()
        .map(|line| count_search(&line.iter().collect::<String>()))
        .sum();

    println!(
        "h: {}, v: {}, d1: {}, d2: {}",
        horizontal, vertical, diagonal_1, diagonal_2
    );

    vertical + horizontal + diagonal_1 + diagonal_2
}


fn transpose<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..v[0].len())
        .map(|i| v.iter().map(|row| row[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn rotate_90deg<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed =  transpose(v);
    transposed.iter_mut().for_each(|row| row.reverse());

    transposed
}


fn get_diagonals<T: Clone + Debug>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let height = v.len();
    let width = v[0].len();
    let mut diagonals = vec![];
    for d in 0..=(height + width - 2) {
        let mut diagonal = vec![];

        for y in 0..=d {
            let x = d - y;
            if y < height && x < width {
                diagonal.push(v[y][x].clone());
            }
        }

        diagonals.push(diagonal);
    }
    diagonals
}

fn count_search(s: &str) -> usize {
    let xre = Regex::new(r"XMAS").unwrap();
    let sre = Regex::new(r"SAMX").unwrap();

    xre.captures_iter(s).count() + sre.captures_iter(s).count()
}

fn solve_second(input: &str) -> usize {
    let lines = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;

    for y in 1..(lines.len() - 1) {
        for x in 1..(lines[0].len() - 1) {
            let center = lines[y][x];
            if center != 'A' {
                continue;
            }
            let upper_left = lines[y-1][x-1];
            let lower_right = lines[y+1][x+1];
            let upper_right = lines[y-1][x+1];
            let lower_left = lines[y+1][x-1];

            if is_ms_match(upper_left, lower_right) && is_ms_match(upper_right, lower_left) {
                count += 1;
            }
        }
    }

    count
}

fn is_ms_match(a: char, b: char) -> bool {
    match (a, b) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false,
    }
}
