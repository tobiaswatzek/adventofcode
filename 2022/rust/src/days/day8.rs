use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("file must be readable");

    let tree_heights = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect(format!("cannot parse {c} as number in line {l}").as_str())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part_one = solve_part_one(&tree_heights);
    let part_two = solve_part_two(&tree_heights);

    (part_one.to_string(), part_two.to_string())
}

fn solve_part_one(tree_heights: &Vec<Vec<u32>>) -> usize {
    // collection[y][x]

    let mut visible_trees = Vec::new();

    for (y, row) in tree_heights.iter().enumerate() {
        for (x, &tree) in row.iter().enumerate() {
            // trees on the outside are always visible
            if y == 0 || y == tree_heights.len() - 1 || x == 0 || x == row.len() - 1 {
                visible_trees.push((y, x));
                continue;
            }

            let left_invisible = row[..x].iter().any(|&t| t >= tree);
            if !left_invisible {
                visible_trees.push((y, x));
                continue;
            }

            let right_invisible = row[x + 1..].iter().any(|&t| t >= tree);
            if !right_invisible {
                visible_trees.push((y, x));
                continue;
            }

            let top_invisible = tree_heights[..y].iter().map(|r| r[x]).any(|t| t >= tree);
            if !top_invisible {
                visible_trees.push((y, x));
                continue;
            }

            let bottom_invisible = tree_heights[y + 1..]
                .iter()
                .map(|r| r[x])
                .any(|t| t >= tree);
            if !bottom_invisible {
                visible_trees.push((y, x));
                continue;
            }
        }
    }

    visible_trees.len()
}

fn solve_part_two(tree_heights: &Vec<Vec<u32>>) -> usize {
    let mut viewing_scores = Vec::new();

    for (y, row) in tree_heights.iter().enumerate() {
        for (x, &tree) in row.iter().enumerate() {
            let left_view = calculate_viewing_distance(row[..x].iter().rev(), tree);
            let right_view = calculate_viewing_distance(row[x + 1..].iter(), tree);
            let top_view =
                calculate_viewing_distance(tree_heights[..y].iter().rev().map(|r| &r[x]), tree);
            let bottom_view =
                calculate_viewing_distance(tree_heights[y + 1..].iter().map(|r| &r[x]), tree);

            let viewing_score = left_view * right_view * top_view * bottom_view;
            viewing_scores.push(viewing_score);
        }
    }

    *viewing_scores
        .iter()
        .max()
        .expect("at least one item expected")
}

fn calculate_viewing_distance<'a, I>(trees: I, tree: u32) -> usize
where
    I: IntoIterator<Item = &'a u32>,
    <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    <I as IntoIterator>::IntoIter: ExactSizeIterator,
{
    let tree_iter = trees.into_iter();
    let num_trees = tree_iter.len();
    tree_iter
        .enumerate()
        .find_map(|(i, &t)| {
            if t < tree && i < num_trees - 1 {
                return None;
            } else {
                Some(i + 1)
            }
        })
        .unwrap_or(0)
}
