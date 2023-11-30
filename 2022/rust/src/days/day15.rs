use std::{fs, hash::Hash, path::PathBuf};

use regex::Regex;

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).unwrap();

    // Sensor at x=1363026, y=2928920: closest beacon is at x=1571469, y=3023534
    let part_one = solve_part_one(&input);
    let part_two = solve_part_two(&input);

    (part_one.to_string(), part_two.to_string())
}

fn solve_part_one(input: &str) -> usize {
    let pairs = parse_pairs(&input);

    let max_distance = pairs.iter().map(|p| p.distance).max().unwrap();

    let min_x = pairs
        .iter()
        .flat_map(|p| [p.beacon.x, p.sensor.x])
        .min()
        .unwrap()
        - max_distance as i64;
    let max_x = pairs
        .iter()
        .flat_map(|p| [p.beacon.x, p.sensor.x])
        .max()
        .unwrap()
        + max_distance as i64;

    let y = 2_000_000;
    (min_x..max_x)
        .filter(|&x| pairs.iter().any(|p| p.is_in_range(&Point { x, y })))
        .count()
}

fn solve_part_two(input: &str) -> usize {
    let pairs = parse_pairs(&input);
    let search_min = Point { x: 0, y: 0 };
    let search_max = Point {
        x: 4_000_000,
        y: 4_000_000,
    };

    let result = pairs
        .iter()
        .flat_map(|p| perimeter_points(&p.sensor, p.distance + 1, (&search_min, &search_max)))
        .find(|point| {
            pairs
                .iter()
                .all(|p| p.sensor.distance_to(&point) > p.distance)
        })
        .unwrap();

    return (result.x * 4_000_000 + result.y) as usize;
}

fn perimeter_points(
    origin: &Point,
    radius: u64,
    (search_min, search_max): (&Point, &Point),
) -> Vec<Point> {
    (origin.y..origin.y + radius as i64)
        .rev()
        .enumerate()
        .chain((origin.y - radius as i64..origin.y).enumerate())
        .flat_map(|(x, y)| {
            if x == 0 {
                vec![Point { x: origin.x, y }]
            } else {
                vec![
                    Point {
                        x: origin.x - x as i64,
                        y,
                    },
                    Point {
                        x: origin.x + x as i64,
                        y,
                    },
                ]
            }
        })
        .filter(|&p| {
            p.x >= search_min.x && p.x <= search_max.x && p.y >= search_min.y && p.y <= search_max.y
        })
        .collect()
}

fn parse_pairs(input: &str) -> Vec<Pair> {
    let line_regex =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();

    input
        .lines()
        .filter_map(|l| {
            line_regex.captures(l).and_then(|c| {
                Some(Pair::new(
                    Point {
                        x: c[1].parse().unwrap(),
                        y: c[2].parse().unwrap(),
                    },
                    Point {
                        x: c[3].parse().unwrap(),
                        y: c[4].parse().unwrap(),
                    },
                ))
            })
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    sensor: Point,
    beacon: Point,
    distance: u64,
}

impl Pair {
    pub fn new(sensor: Point, beacon: Point) -> Self {
        let distance = sensor.distance_to(&beacon);

        Self {
            sensor,
            beacon,
            distance,
        }
    }

    pub fn is_in_range(&self, other: &Point) -> bool {
        &self.beacon != other && self.sensor.distance_to(other) <= self.distance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs())
            .try_into()
            .unwrap()
    }
}
