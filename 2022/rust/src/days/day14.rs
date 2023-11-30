use std::{collections::HashSet, fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).unwrap();

    let part_one = solve_part_one(&input);
    let part_two = solve_part_two(&input);

    return (part_one.to_string(), part_two.to_string());
}

pub fn solve_part_one(input: &str) -> usize {
    let rocks: HashSet<Point> = parse_rocks(&input);

    let max_y = rocks.iter().max_by_key(|&p| p.y).unwrap().y;

    let sand_origin = Point { x: 500, y: 0 };
    let mut sand: HashSet<Point> = HashSet::new();
    loop {
        let mut sand_grain = sand_origin;

        while !sand.contains(&sand_grain) && sand_grain.y < max_y {
            if let Some(next) = next_free_space(&sand_grain, &rocks, &sand, None) {
                sand_grain = next;
                continue;
            }

            sand.insert(sand_grain);
        }

        if sand_grain.y >= max_y {
            break;
        }
    }

    return sand.len();
}

pub fn solve_part_two(input: &str) -> usize {
    let rocks: HashSet<Point> = parse_rocks(&input);

    let max_y = rocks.iter().max_by_key(|&p| p.y).unwrap().y;
    let floor = max_y + 2;

    let sand_origin = Point { x: 500, y: 0 };
    let mut sand: HashSet<Point> = HashSet::new();
    loop {
        let mut sand_grain = sand_origin;

        while !sand.contains(&sand_grain) {
            if let Some(next) = next_free_space(&sand_grain, &rocks, &sand, Some(floor)) {
                sand_grain = next;
                continue;
            }

            sand.insert(sand_grain);
        }

        if sand.contains(&sand_origin) {
            break;
        }
    }

    return sand.len();
}

fn next_free_space(
    current: &Point,
    rocks: &HashSet<Point>,
    sand: &HashSet<Point>,
    floor: Option<usize>,
) -> Option<Point> {
    let below = current.below();
    if !rocks.contains(&below)
        && !sand.contains(&below)
        && (floor.is_none() || floor.unwrap() > below.y)
    {
        return Some(below);
    }

    let left_down = current.left_down();
    if !rocks.contains(&left_down)
        && !sand.contains(&left_down)
        && (floor.is_none() || floor.unwrap() > left_down.y)
    {
        return Some(left_down);
    }

    let right_down = current.right_down();
    if !rocks.contains(&right_down)
        && !sand.contains(&right_down)
        && (floor.is_none() || floor.unwrap() > right_down.y)
    {
        return Some(right_down);
    }

    None
}

fn parse_rocks(input: &str) -> HashSet<Point> {
    input
        .lines()
        .flat_map(|l| {
            let pairs = l
                .split(" -> ")
                .filter_map(|s| {
                    if s.is_empty() {
                        return None;
                    }

                    let splits = s.split(",").collect::<Vec<_>>();

                    Some(Point {
                        x: splits[0].parse::<usize>().unwrap(),
                        y: splits[1].parse::<usize>().unwrap(),
                    })
                })
                .collect::<Vec<_>>();

            let mut formations = vec![];
            for p in pairs.windows(2) {
                formations.push(p[0]);
                formations.push(p[1]);
                match p {
                    [l, r] if l.x == r.x => {
                        let max_y = std::cmp::max(l.y, r.y);
                        let min_y = std::cmp::min(l.y, r.y);
                        (min_y..=max_y).for_each(|y| {
                            formations.push(Point { x: l.x, y });
                        });
                    }
                    [l, r] if l.y == r.y => {
                        let max_x = std::cmp::max(l.x, r.x);
                        let min_x = std::cmp::min(l.x, r.x);
                        (min_x..=max_x).for_each(|x| {
                            formations.push(Point { x, y: l.y });
                        });
                    }
                    _ => panic!("unexpected window {p:?}"),
                }
            }

            formations
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn below(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left_down(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    pub fn right_down(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

#[test]
fn test_rock_formation_parsing() {
    let expected_rocks = vec![
        Point { x: 496, y: 6 },
        Point { x: 497, y: 6 },
        Point { x: 498, y: 6 },
        Point { x: 498, y: 5 },
        Point { x: 498, y: 4 },
        Point { x: 503, y: 4 },
        Point { x: 502, y: 4 },
        Point { x: 502, y: 5 },
        Point { x: 502, y: 6 },
        Point { x: 502, y: 7 },
        Point { x: 502, y: 8 },
        Point { x: 502, y: 9 },
        Point { x: 501, y: 9 },
        Point { x: 500, y: 9 },
        Point { x: 499, y: 9 },
        Point { x: 498, y: 9 },
        Point { x: 497, y: 9 },
        Point { x: 496, y: 9 },
        Point { x: 495, y: 9 },
        Point { x: 494, y: 9 },
    ];

    let input = concat!(
        "498,4 -> 498,6 -> 496,6\n",
        "503,4 -> 502,4 -> 502,9 -> 494,9\n"
    );

    let actual = parse_rocks(&input);

    assert_eq!(
        actual.len(),
        expected_rocks.len(),
        "actual len {} was not expected len {}",
        actual.len(),
        expected_rocks.len()
    );
    for expected in expected_rocks {
        assert!(
            actual.contains(&expected),
            "expected point {expected:?} was not in actual"
        );
    }
}
