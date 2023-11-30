use std::{collections::HashSet, fs, hash::Hash, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).unwrap();
    let directions = parse_directions(&input);
    let part_one = solve_part_one(&directions);
    let part_two = solve_part_two(&directions);

    (part_one.to_string(), part_two.to_string())
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map_while(|c| match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect()
}

fn solve_part_one(directions: &[Direction]) -> u64 {
    let heights_per_round = run_simulation(directions, 2022);

    *heights_per_round.last().unwrap()
}

fn solve_part_two(directions: &[Direction]) -> u64 {
    let num_of_directions = directions.len() as u64;
    let heights_per_round = run_simulation(directions, num_of_directions as usize);
    let max = *heights_per_round.last().unwrap();
    let num_of_rocks: u64 = 2022;
    let max_multiplicand = num_of_rocks % num_of_directions;
    let height_without_rest = max * max_multiplicand;
    //println!("{num_of_directions} {max} {height_without_rest} {}", heights_per_round[22]);
    //println!("{heights_per_round:#?}");
    0
}

fn run_simulation(directions: &[Direction], num_of_rocks: usize) -> Vec<u64> {
    let direction_count = directions.len();
    let width = 7;

    let left_distance = 2;
    let bottom_distance = 4;

    let mut count: usize = 0;

    let mut height_per_round: Vec<u64> = Vec::with_capacity(num_of_rocks);
    let mut rocks: HashSet<Point> = HashSet::new();
    for n in 0..num_of_rocks {
        let shape = shape_for_index(n);
        let mut vertices = shape.vertices();
        vertices = move_x(left_distance, &vertices);
        let offset_y = rocks
            .iter()
            .max_by_key(|&p| p.y)
            .and_then(|p| Some(p.y))
            .unwrap_or(-1)
            + bottom_distance;
        // println!("offset {offset_y}");
        vertices = move_y(offset_y, &vertices);

        // println!("{vertices:?}");

        loop {
            let push_x = match &directions[count % direction_count] {
                Direction::Left => -1,
                Direction::Right => 1,
            };

            // println!("{push_x:?}");

            let shifted_vertices = move_x(push_x, &vertices);
            let down_vertices = if shifted_vertices
                .iter()
                .all(|p| (p.x >= 0 && p.x < width) && !rocks.contains(p))
            {
                //  println!("shifted");
                move_y(-1, &shifted_vertices)
            } else {
                // println!("unshifted");
                move_y(-1, &vertices)
            };

            if down_vertices.iter().any(|p| rocks.contains(p) || p.y < 0) {
                // touching the ground :o
                vertices = move_y(1, &down_vertices);
                count += 1;
                break;
            }

            vertices = down_vertices;
            count += 1;
            // println!("{vertices:?} {count}");
        }
        for p in vertices {
            rocks.insert(p);
        }

        let height: u64 = (rocks.iter().max_by_key(|&p| p.y).unwrap().y + 1) as u64;

        height_per_round.push(height);
    }

    height_per_round
}

fn move_y(y: i64, vertices: &Vec<Point>) -> Vec<Point> {
    vertices.iter().map(|p| Point::new(p.x, p.y + y)).collect()
}

fn move_x(x: i64, vertices: &Vec<Point>) -> Vec<Point> {
    vertices.iter().map(|p| Point::new(p.x + x, p.y)).collect()
}

fn shape_for_index(index: usize) -> Shape {
    match index % 5 {
        0 => Shape::Line,
        1 => Shape::Cross,
        2 => Shape::ReverseL,
        3 => Shape::Pipe,
        4 => Shape::Square,
        _ => panic!("the end is near math does no longer work"),
    }
}

fn print_board(rocks: &HashSet<Point>) {
    let max_y = rocks.iter().max_by_key(|&p| p.y).unwrap().y;
    println!();
    for y in (0..=max_y).rev() {
        print!("|");
        for x in 0..7 {
            if rocks.contains(&Point::new(x, y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!("|");
    }
    println!("+-------+")
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    Line = 0,
    Cross = 1,
    ReverseL = 2,
    Pipe = 3,
    Square = 4,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl Shape {
    pub fn vertices(&self) -> Vec<Point> {
        match self {
            Shape::Line => vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
            ],
            Shape::Cross => vec![
                Point::new(1, 2),
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(1, 0),
            ],
            Shape::ReverseL => vec![
                Point::new(2, 2),
                Point::new(2, 1),
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
            ],
            Shape::Pipe => vec![
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
            ],
            Shape::Square => vec![
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(0, 0),
                Point::new(1, 0),
            ],
        }
    }
}
