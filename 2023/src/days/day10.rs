use std::{fs, iter, path::PathBuf};

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
    let maze: Vec<Vec<char>> = parse_maze(input);
    let start = find_start(&maze);
    let mut route = find_route(&start, &maze);
    route.insert(0, start);

    let xs: Vec<usize> = route.iter().map(|p| p.x).collect();
    let x_min = *xs.iter().min().unwrap();
    let x_max = *xs.iter().max().unwrap();
    let ys: Vec<usize> = route.iter().map(|p| p.y).collect();
    let y_min = *ys.iter().min().unwrap();
    let y_max = *ys.iter().max().unwrap();

    let candidates: Vec<Point> = maze
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, _)| {
                    if x < x_min || x > x_max || y < y_min || y > y_max {
                        None
                    } else {
                        Some(Point { x, y })
                    }
                })
                .filter(|p| !route.contains(p))
        })
        .collect();

    let reduced_route = reduce_route(route);

    let points: Vec<&Point> = reduced_route
        .iter()
        .chain(iter::once(&reduced_route[0]))
        .collect();

    let sides: Vec<Vector> = points
        .windows(2)
        .map(|w| match w {
            [a, b] => Vector(
                VectorPoint {
                    x: a.x_int(),
                    y: a.y_int(),
                },
                VectorPoint {
                    x: b.x_int(),
                    y: b.y_int(),
                },
            ),
            _ => unreachable!("windows of size 2 expected"),
        })
        .collect();

    let x_outside: i64 = TryInto::<i64>::try_into(x_min).unwrap() - 1;
    let y_outside: i64 = TryInto::<i64>::try_into(y_min).unwrap() - 1;
    let points_inside: Vec<&Point> = candidates
        .iter()
        .filter(|candidate| {
            let outside: (i64, i64) = (x_outside, candidate.y_int());
            let ray = Vector(
                // todo: really need to find out which number to use
                VectorPoint { x: -1, y: -22 },
                VectorPoint {
                    x: candidate.x_int(),
                    y: candidate.y_int(),
                },
            );

            let intersections: Vec<&Vector> = sides
                .iter()
                .filter(|side| are_intersecting(side, &ray))
                .collect();

            let is_match = intersections.len() % 2 != 0;

            is_match
        })
        .collect();

    let inside_count = points_inside.len();

    inside_count
}

fn reduce_route(route: Vec<Point>) -> Vec<Point> {
    let mut reduced_route = vec![];

    let j = route.len() - 1;
    let mut p = route[j - 1];
    let mut q = route[j];

    for r in &route {
        let ax = q.x_int() - p.x_int();
        let ay = q.y_int() - p.y_int();
        let bx = r.x_int() - q.x_int();
        let by = r.y_int() - q.y_int();
        let cross_product = ax * by - ay * bx;
        let dot_product = ax * bx + ay * by;

        if cross_product != 0 || dot_product <= 0 {
            reduced_route.push(q);
        }

        p = q;
        q = *r;
    }

    reduced_route
}

fn are_intersecting(v1: &Vector, v2: &Vector) -> bool {
    let a1 = v1.1.y - v1.0.y;
    let b1 = v1.0.x - v1.1.x;
    let c1 = (v1.1.x * v1.0.y) - (v1.0.x * v1.1.y);

    let d1 = (a1 * v2.0.x) + (b1 * v2.0.y) + c1;
    let d2 = (a1 * v2.1.x) + (b1 * v2.1.y) + c1;

    if (d1 > 0 && d2 > 0) || (d1 < 0 && d2 < 0) {
        return false;
    }

    let a2 = v2.1.y - v2.0.y;
    let b2 = v2.0.x - v2.1.x;
    let c2 = (v2.1.x * v2.0.y) - (v2.0.x * v2.1.y);

    let d3 = (a2 * v1.0.x) + (b2 * v1.0.y) + c2;
    let d4 = (a2 * v1.1.x) + (b2 * v1.1.y) + c2;

    if (d3 > 0 && d4 > 0) || (d3 < 0 && d4 < 0) {
        return false;
    }

    if ((a1 * b2) - (a2 * b1)) == 0 {
        return false;
    }

    true
}

struct Vector(VectorPoint, VectorPoint);

struct VectorPoint {
    x: i64,
    y: i64,
}

fn is_between_points(to_test: (i64, i64), start: (i64, i64), end: (i64, i64)) -> bool {
    let dx_line = start.0 - end.0;
    let dy_line = start.1 - end.1;

    let dx_side = to_test.0 - end.0;
    let dy_side = to_test.1 - end.1;

    let cross_product = dx_side * dy_line - dy_side * dx_line;

    if cross_product != 0 {
        return false;
    }

    let dot_product =
        (to_test.0 - end.0) * (start.0 - end.0) + (to_test.1 - end.1) * (start.1 - end.1);
    if dot_product < 0 {
        return false;
    }
    let squared_length =
        (start.0 - end.0) * (start.0 - end.0) + (start.1 - end.1) * (start.1 - end.1);

    if dot_product > squared_length {
        return false;
    }

    true
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn x_int(&self) -> i64 {
        self.x.try_into().unwrap()
    }

    fn y_int(&self) -> i64 {
        self.y.try_into().unwrap()
    }

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
