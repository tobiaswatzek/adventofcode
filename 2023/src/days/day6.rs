use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> u64 {
    let races = parse_races_first(&input);

    races.iter().map(|r| number_wins(r)).product()
}

fn number_wins(race: &Race) -> u64 {
    let mut number_of_wins = 0;
    for time_pressed in 1..race.time {
        let travel_time = race.time - time_pressed;
        let distance = time_pressed * travel_time;
        if number_of_wins > 0 && distance <= race.distance {
            break;
        }
        if distance > race.distance {
            number_of_wins += 1;
        }
    }

    number_of_wins
}

fn solve_second(input: &str) -> u64 {
    let race = parse_race_second(input);
    number_wins(&race)
}

fn parse_races_first(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let times = lines[0][6..].split(' ').filter_map(|s| {
        if s.is_empty() {
            None
        } else {
            s.parse::<u64>().ok()
        }
    });
    let distances = lines[1][10..].split(' ').filter_map(|s| {
        if s.is_empty() {
            None
        } else {
            s.parse::<u64>().ok()
        }
    });

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_race_second(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();
    let time = lines[0][6..].replace(' ', "").parse().unwrap();
    let distance = lines[1][10..].replace(' ', "").parse().unwrap();

    Race { time, distance }
}

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    distance: u64,
}
