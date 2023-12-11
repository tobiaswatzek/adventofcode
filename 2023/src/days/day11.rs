use std::{collections::HashSet, fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> i64 {
    let universe: Vec<Vec<Observation>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Observation::Galaxy,
                    '.' => Observation::Void,
                    _ => unreachable!("only . and # expected"),
                })
                .collect::<Vec<Observation>>()
        })
        .collect();

    let mut expanded_universe: Vec<Vec<Observation>> = universe
        .into_iter()
        .flat_map(|row| {
            if row.iter().all(|o| *o == Observation::Void) {
                vec![row.clone(), row.clone()]
            } else {
                vec![row]
            }
        })
        .collect();

    expanded_universe = transpose(expanded_universe)
        .into_iter()
        .flat_map(|row| {
            if row.iter().all(|o| *o == Observation::Void) {
                vec![row.clone(), row.clone()]
            } else {
                vec![row]
            }
        })
        .collect();

    expanded_universe = transpose(expanded_universe);

    let all_galaxies: Vec<Point> = expanded_universe
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, o)| match o {
                Observation::Void => None,
                Observation::Galaxy => Some(Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                }),
            })
        })
        .collect();

    let galaxy_pairs: HashSet<(&Point, &Point)> = all_galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            all_galaxies[i + 1..]
                .iter()
                .map(move |b| if a < b { (a, b) } else { (b, a) })
        })
        .collect();

    let sum_distances = galaxy_pairs
        .iter()
        .map(|(a, b)| taxicab_distance(a, b))
        .sum();

    sum_distances
}

fn taxicab_distance(a: &Point, b: &Point) -> i64 {
    let d = (a.x - b.x).abs() + (a.y - b.y).abs();

    d
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Observation {
    Void,
    Galaxy,
}

fn solve_second(input: &str) -> usize {
    0
}
