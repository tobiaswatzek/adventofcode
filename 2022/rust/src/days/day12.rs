use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::PathBuf,
    time::Instant,
};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).unwrap();
    let grid = parse_point_value_grid(&input);


    let part_one = solve_part_one(&grid);
    let before = Instant::now();
    let part_two = solve_part_two(&grid);
    println!("Elapsed: {:.2?}", before.elapsed());

    return (part_one.to_string(), part_two.to_string());
}

fn solve_part_one(grid: &Vec<Vec<PointValue>>) -> usize {
    let nodes = connect_nodes(&grid);
    let start_end = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, PointValue(_, p))| match p {
                    PointType::Start => Some((PointType::Start, Point { x, y })),
                    PointType::End => Some((PointType::End, Point { x, y })),
                    _ => None,
                })
        })
        .collect::<Vec<_>>();

    let start = start_end
        .iter()
        .find(|(t, _)| t == &PointType::Start)
        .unwrap()
        .1;
    let end = start_end
        .iter()
        .find(|(t, _)| t == &PointType::End)
        .unwrap()
        .1;

    let height_map = HeightMap {
        start,
        end,
        nodes: &nodes,
    };

    bfs(&height_map).unwrap()
}

fn solve_part_two(grid: &Vec<Vec<PointValue>>) -> usize {
    let nodes = connect_nodes(&grid);
    let start_end = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, PointValue(v, t))| {
                    if v == &('a' as u32) {
                        return Some((PointType::Start, Point { x, y }));
                    }

                    if t == &PointType::End {
                        return Some((PointType::End, Point { x, y }));
                    }

                    None
                })
        })
        .collect::<Vec<_>>();

    let starts = start_end.iter().filter_map(|(t, p)| match t {
        PointType::Start => Some(p),
        _ => None,
    });

    let end = start_end
        .iter()
        .find(|(t, _)| t == &PointType::End)
        .unwrap()
        .1;

    starts
        .filter_map(|&s| {
            let hm = HeightMap {
                start: s,
                end,
                nodes: &nodes,
            };
            bfs(&hm)
        })
        .min()
        .unwrap()
}

fn bfs(height_map: &HeightMap) -> Option<usize> {
    let mut predecessors = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(height_map.start);
    visited.insert(height_map.start);

    let mut found_end = false;

    while let Some(point) = queue.pop_front() {
        if point == height_map.end {
            found_end = true;
            break;
        }

        for &child in height_map.nodes.get(&point).unwrap() {
            if !visited.contains(&child) {
                queue.push_back(child);
                visited.insert(child);
                predecessors.insert(child, point);
            }
        }
    }

    if !found_end {
        return None;
    }

    let mut key = height_map.end;
    let mut path = Vec::new();
    path.push(key);
    while let Some(&p) = predecessors.get(&key) {
        path.push(p);
        key = p;
    }

    Some(path.len() - 1)
}

fn parse_point_value_grid(input: &str) -> Vec<Vec<PointValue>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => PointValue('a' as u32, PointType::Start),
                    'E' => PointValue('z' as u32, PointType::End),
                    n => PointValue(n as u32, PointType::None),
                })
                .collect::<Vec<PointValue>>()
        })
        .collect::<Vec<Vec<PointValue>>>()
}

fn connect_nodes(grid: &Vec<Vec<PointValue>>) -> HashMap<Point, Vec<Point>> {
    let mut nodes: HashMap<Point, Vec<Point>> = HashMap::with_capacity(grid.len() * grid[0].len());

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let point = Point { x, y };
            nodes.insert(point, build_nodes(&point, &grid));
        }
    }

    nodes
}

fn build_nodes(current: &Point, point_value_grid: &Vec<Vec<PointValue>>) -> Vec<Point> {
    let PointValue(current_weight, _) = point_value_grid[current.y][current.x];

    current
        .neighbors()
        .iter()
        .filter_map(|&neighbor| match neighbor {
            Some(p) => point_value_grid
                .get(p.y)
                .and_then(|row| row.get(p.x))
                .and_then(|&PointValue(w, _)| {
                    if w <= current_weight + 1 {
                        return Some(p);
                    }

                    None
                }),
            None => None,
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn neighbors(&self) -> [Option<Self>; 4] {
        [self.left(), self.top(), self.right(), self.bottom()]
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Point {
            x: self.x - 1,
            y: self.y,
        })
    }

    pub fn right(&self) -> Option<Self> {
        Some(Point {
            x: self.x + 1,
            y: self.y,
        })
    }

    pub fn top(&self) -> Option<Self> {
        Some(Point {
            x: self.x,
            y: self.y + 1,
        })
    }

    pub fn bottom(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Point {
            x: self.x,
            y: self.y - 1,
        })
    }
}

#[derive(Debug, Clone)]
struct HeightMap<'a> {
    start: Point,
    end: Point,
    nodes: &'a HashMap<Point, Vec<Point>>,
}

#[derive(Debug, Clone, PartialEq)]
enum PointType {
    None,
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq)]
struct PointValue(u32, PointType);
