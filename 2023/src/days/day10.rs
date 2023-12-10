use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> usize {
    let maze: Vec<Vec<char>> = parse_maze(input);
    let start = find_start(&maze);
    let route = find_route(&start, &maze);
    let result = route.len().div_ceil(2);

    result
}

fn solve_second(input: &str) -> usize {
    0   
}

fn parse_maze(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_start(maze: &Vec<Vec<char>>) -> Point {
    maze.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| match c {
                    'S' => Some(x),
                    _ => None,
                })
                .and_then(|x| Some(Point { x, y }))
        })
        .expect("there must be a start position")
}



fn find_route(start: &Point, maze: &Vec<Vec<char>>) -> Vec<Point> {
    let loop_starts = connections_from_start(&start, &maze);

    let mut route = vec![];
    let mut previous = *start;
    let mut current = loop_starts.0;
    loop {
        if current == *start {
            break;
        }
        route.push(current);
        let connections = connections_from(&maze[current.y][current.x], &current);
        let next = connections
            .and_then(|c| match c {
                (p, n) if p == previous => Some(n),
                (n, p) if p == previous => Some(n),
                _ => None,
            })
            .unwrap();
        previous = current;
        current = next;
    }
    route
}

fn connections_from_start(start: &Point, maze: &Vec<Vec<char>>) -> (Point, Point) {
    let possible_connections = [start.north(), start.east(), start.south(), start.west()];
    let loop_starts: Vec<Point> = possible_connections
        .iter()
        .filter_map(|op| {
            op.and_then(|p| {
                connections_from(&maze[p.y][p.x], &p).and_then(|(first, second)| {
                    if first == *start || second == *start {
                        Some(p)
                    } else {
                        None
                    }
                })
            })
        })
        .collect();

    if let [a, b] = loop_starts[..] {
        (a, b)
    } else {
        panic!("there must be exactly two connections")
    }
}

fn connections_from(pipe: &char, at: &Point) -> Option<(Point, Point)> {
    let connections = match pipe {
        '|' => Some((at.north(), at.south())),
        '-' => Some((at.west(), at.east())),
        'L' => Some((at.north(), at.east())),
        'J' => Some((at.north(), at.west())),
        '7' => Some((at.south(), at.west())),
        'F' => Some((at.south(), at.east())),
        _ => None,
    };

    connections.and_then(|c| match c {
        (Some(a), Some(b)) => Some((a, b)),
        (_, None) | (None, _) => None,
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn north(&self) -> Option<Point> {
        if self.y == 0 {
            None
        } else {
            Some(Point {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    fn east(&self) -> Option<Point> {
        Some(Point {
            x: self.x + 1,
            y: self.y,
        })
    }

    fn south(&self) -> Option<Point> {
        Some(Point {
            x: self.x,
            y: self.y + 1,
        })
    }

    fn west(&self) -> Option<Point> {
        if self.x == 0 {
            None
        } else {
            Some(Point {
                x: self.x - 1,
                y: self.y,
            })
        }
    }
}
