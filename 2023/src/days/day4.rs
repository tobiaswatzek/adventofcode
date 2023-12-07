use std::{collections::HashSet, fs, path::PathBuf, usize};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> i32 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let cards = l[8..]
                .split('|')
                .map(|nums| {
                    nums.split(' ')
                        .filter_map(|n| n.trim().parse::<i32>().ok())
                        .collect::<HashSet<i32>>()
                })
                .collect::<Vec<_>>();
            let (winning_numbers, scratched_numbers) = match &cards[..] {
                [first, second] => (first, second),
                _ => unreachable!(),
            };

            let count: u32 = winning_numbers
                .intersection(&scratched_numbers)
                .count()
                .try_into()
                .unwrap();

            if count == 0 {
                return 0;
            }

            let points: i32 = 2_i32.pow(count - 1);

            points
        })
        .sum()
}

fn solve_second(input: &str) -> usize {
    let cards: Vec<Card> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .map(|(id, l)| {
            let card = l[8..]
                .split('|')
                .map(|nums| {
                    nums.split(' ')
                        .filter_map(|n| n.trim().parse::<i32>().ok())
                        .collect::<HashSet<i32>>()
                })
                .collect::<Vec<_>>();
            let (winning_numbers, scratched_numbers) = match &card[..] {
                [first, second] => (first, second),
                _ => unreachable!(),
            };

            let count = winning_numbers.intersection(&scratched_numbers).count();

            Card {
                id,
                won_copies: (id + 1..id + count + 1).collect(),
            }
        })
        .collect();

    let sum_cards = cards
        .iter()
        .fold(0, |acc, card| acc + evaluate_cards(&card.id, &cards));

    sum_cards
}

fn evaluate_cards(current_card: &usize, all_cards: &Vec<Card>) -> usize {
    let mut count = 1;

    for won_copy in &all_cards[*current_card].won_copies {
        count += evaluate_cards(won_copy, all_cards);
    }

    count
}

#[derive(Debug)]
struct Card {
    id: usize,
    won_copies: Vec<usize>,
}
