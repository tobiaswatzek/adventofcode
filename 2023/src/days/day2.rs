use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);
    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> i32 {
    let games = parse_games(input);

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    games.iter().fold(0, |acc, g| {
        if g.rounds
            .iter()
            .any(|r| r.red > max_red || r.green > max_green || r.blue > max_blue)
        {
            acc
        } else {
            acc + g.id
        }
    })
}

fn solve_second(input: &str) -> i32 {
    let games = parse_games(input);

    games.iter().fold(0, |acc, g| {
        let max_red = g.rounds.iter().max_by_key(|r| r.red).unwrap().red;
        let max_green = g.rounds.iter().max_by_key(|r| r.green).unwrap().green;
        let max_blue = g.rounds.iter().max_by_key(|r| r.blue).unwrap().blue;

        acc + (max_red * max_green * max_blue)
    })
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let s = l[5..].to_owned();
            let (id, game_str) = s.split_once(':').expect("must contain colon");
            let game_id: i32 = id.parse().expect("id must be a number");
            let rounds = game_str
                .split(';')
                .map(|rs| {
                    let (r, g, b) = rs.split(',').fold((0, 0, 0), |(r, g, b), c| {
                        match c.trim().split(' ').collect::<Vec<_>>()[..] {
                            [n, "red"] => (n.parse().unwrap(), g, b),
                            [n, "green"] => (r, n.parse().unwrap(), b),
                            [n, "blue"] => (r, g, n.parse().unwrap()),
                            _ => (r, g, b),
                        }
                    });

                    Round {
                        red: r,
                        green: g,
                        blue: b,
                    }
                })
                .collect();

            Game {
                id: game_id,
                rounds,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}
