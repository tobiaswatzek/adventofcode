use std::{
    collections::HashMap,
    fmt::{self, Display},
    fs,
    path::PathBuf,
};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> u32 {
    let mut games: Vec<Game> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let cards: [Card; 5] = l[..5].chars().map(|c| char_to_card(&c)).collect::<Vec<_>>()[..]
                .try_into()
                .unwrap();
            let bid: u32 = l[6..].parse().unwrap();

            Game {
                bid,
                hand: Hand::new(cards),
            }
        })
        .collect();

    games.sort_unstable_by(|lhs, rhs| rhs.hand.cmp(&lhs.hand));

    games
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let rank: u32 = (i + 1).try_into().unwrap();
            let winnings = rank * g.bid;
            //println!("{rank} {} {winnings} {:?}", g.bid, g.hand);
            winnings
        })
        .sum()
}

fn solve_second(input: &str) -> u64 {
    0
}

fn char_to_card(c: &char) -> Card {
    match c {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        n if n.is_numeric() => Card::Number(n.to_digit(10).unwrap().try_into().unwrap()),
        _ => panic!("unsupported char"),
    }
}

#[derive(Debug)]
struct Game {
    hand: Hand,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        other.cards.cmp(&self.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        other.cards.partial_cmp(&self.cards)
    }
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        let hand_type = Self::find_hand_type(&cards);

        Self { cards, hand_type }
    }

    fn find_hand_type(cards: &[Card; 5]) -> HandType {
        /*
           Five of a kind, where all five cards have the same label: AAAAA
           Four of a kind, where four cards have the same label and one card has a different label: AA8AA
           Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
           Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
           Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
           One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
           High card, where all cards' labels are distinct: 23456
        */
        let mut groups: HashMap<Card, u8> = HashMap::new();
        for card in cards {
            if let Some(count) = groups.get_mut(card) {
                *count += 1
            } else {
                groups.insert(*card, 1);
            }
        }

        let h = match groups.len() {
            1 => HandType::FiveOfAKind,
            2 => match groups.values().max().unwrap() {
                4 => HandType::FourOfAKind,
                3 => HandType::FullHouse,
                _ => unreachable!("no other combinations possible"),
            },
            3 => match groups.values().max().unwrap() {
                3 => HandType::ThreeOfAKind,
                2 => HandType::TwoPair,
                _ => unreachable!("no other combinations possible"),
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!("all cases are exhausted when using five cards"),
        };

        h
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandType::FiveOfAKind => write!(f, "FiveOfAKind"),
            HandType::FourOfAKind => write!(f, "FourOfAKind"),
            HandType::FullHouse => write!(f, "FullHouse"),
            HandType::ThreeOfAKind => write!(f, "ThreeOfAKind"),
            HandType::TwoPair => write!(f, "TwoPair"),
            HandType::OnePair => write!(f, "OnePair"),
            HandType::HighCard => write!(f, "HighCard"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(u8),
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Card::A => write!(f, "A"),
            Card::K => write!(f, "K"),
            Card::Q => write!(f, "Q"),
            Card::J => write!(f, "J"),
            Card::T => write!(f, "T"),
            Card::Number(n) => write!(f, "{n}"),
        }
    }
}

impl Card {
    fn rank(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::Number(n) => *n,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}
