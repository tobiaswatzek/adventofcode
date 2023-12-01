use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = 0; // solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> i32 {
    let sum: i32 = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let first_number = l
                .chars()
                .find(|c| c.is_numeric())
                .expect("there must be a number");
            let last_number = l
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .expect("there must be a last number");

            let number = format!("{}{}", first_number, last_number)
                .parse::<i32>()
                .expect("must be a number");

            number
        })
        .sum();
    sum
}

fn solve_second(input: &str) -> i32 {
    let number_strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let sum: i32 = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mod_l = number_strings
                .iter()
                .enumerate()
                .filter_map(|(i, e)| l.find(e).and_then(|fi| Some((i, e, fi))))
                .min_by(|a, b| a.2.cmp(&b.2))
                .and_then(|(i, e, fi)| {
                    Some(
                        l.split_once(e)
                            .and_then(|(a, b)| Some(format!("{}{}{}", a, i + 1, b)))
                            .expect("must be splitable"),
                    )
                })
                .unwrap_or(l.to_owned());

            let number_line = number_strings
                .iter()
                .enumerate()
                .filter_map(|(i, e)| mod_l.find(e).and_then(|fi| Some((i, e, fi))))
                .max_by(|a, b| a.2.cmp(&b.2)).and_then(|(i, e, fi)| {
                    Some(
                        mod_l.rsplit_once(e)
                            .and_then(|(a, b)| Some(format!("{}{}{}", a, i + 1, b)))
                            .expect("must be splitable"),
                    )
                })
                .unwrap_or(mod_l);

            let first_number = number_line
                .chars()
                .find(|c| c.is_numeric())
                .expect("there must be a number");
            let last_number = number_line
                .chars()
                .rfind(|c| c.is_numeric())
                .expect("there must be a last number");

            let number = format!("{}{}", first_number, last_number)
                .parse::<i32>()
                .expect("must be a number");

            number
        })
        .sum();
    sum
}

fn replace(line: &str, find_index: usize, number: usize, number_str: &str) -> String {
    let mut copy = line.to_owned();
    copy.replace_range(
        find_index..find_index + number_str.len(),
        number.to_string().as_str(),
    );

    copy
}
