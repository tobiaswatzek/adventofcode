use core::panic;
use std::{cell::RefCell, fmt::Display, fs, path::PathBuf, rc::Rc, str::Chars};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).unwrap();
    let part_one = solve_part_one(&input);
    let part_two = solve_part_two(&input);

    return (part_one.to_string(), part_two.to_string());
}

fn solve_part_one(input: &str) -> usize {
    let pairs = parse_pairs(input);

    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| {
            let sorted = l.is_sorted(r);

            match sorted {
                Sort::Sorted => Some(i + 1),
                _ => None,
            }
        })
        .sum()
}

fn solve_part_two(input: &str) -> usize {
    let divider_one = parse_packet_data("[[2]]");
    let divider_two = parse_packet_data("[[6]]");
    let mut packets = input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }

            Some(parse_packet_data(l))
        })
        .chain([Rc::clone(&divider_one), Rc::clone(&divider_two)])
        .collect::<Vec<_>>();

    packets.sort_unstable();

    let pos_div_one = packets
        .iter()
        .position(|p| Rc::ptr_eq(p, &divider_one))
        .unwrap()
        + 1;
    let pos_div_two = packets
        .iter()
        .position(|p| Rc::ptr_eq(p, &divider_two))
        .unwrap()
        + 1;

    pos_div_one * pos_div_two
}

fn parse_pairs(input: &str) -> Vec<(Rc<PacketData>, Rc<PacketData>)> {
    input
        .split("\n\n")
        .map(|str_pair| {
            let split_pair = str_pair.split("\n").collect::<Vec<_>>();

            let left = parse_packet_data(split_pair[0]);
            let right = parse_packet_data(split_pair[1]);

            (left, right)
        })
        .collect()
}

fn parse_packet_data(data_str: &str) -> Rc<PacketData> {
    let tokens = parse_tokens(data_str.chars());
    let (packet_data, _) = tokens_to_data(&tokens, None);
    packet_data.expect(format!("could not parse tokens from {data_str}").as_str())
}

fn tokens_to_data(
    tokens: &[Token],
    packet_data: Option<Rc<PacketData>>,
) -> (Option<Rc<PacketData>>, &[Token]) {
    let split = tokens.split_first();
    if split.is_none() {
        return (packet_data, &[]);
    }

    let t = split.unwrap().0;
    let tail = split.unwrap().1;
    if let Some(inner) = &packet_data {
        match (t, inner.as_ref()) {
            (Token::Comma, _) => return tokens_to_data(tail, packet_data),
            (Token::Number(n), PacketData::List(l)) => {
                let number = PacketData::new_number(n.parse().unwrap());
                l.borrow_mut().push(number);
                return tokens_to_data(tail, packet_data);
            }
            (Token::ListStart, PacketData::List(parent_to_be)) => {
                let (child, new_tail) = tokens_to_data(tail, Some(PacketData::new_list(vec![])));
                parent_to_be.borrow_mut().push(child.unwrap());
                return tokens_to_data(new_tail, packet_data);
            }
            (Token::ListEnd, PacketData::List(_)) => {
                return (packet_data, tail);
            }
            _ => panic!("unexpected token combination {t:?}"),
        }
    } else if t == &Token::ListStart && packet_data.is_none() {
        return tokens_to_data(tail, Some(PacketData::new_list(vec![])));
    }

    panic!("unexpected token combination {t:?}")
}

fn parse_tokens(chars: Chars) -> Vec<Token> {
    let raw_tokens = chars.map(|c| match c {
        '[' => Token::ListStart,
        ']' => Token::ListEnd,
        ',' => Token::Comma,
        n if n.is_numeric() => Token::Number(n.to_string()),
        _ => panic!("unexpected character '{c}' encountered"),
    });
    let mut acc = vec![];

    for current in raw_tokens {
        let prev = acc.last();

        match (current, prev) {
            (Token::Number(c), Some(Token::Number(p))) => {
                let t = Token::Number(p.clone() + &c);
                let last_idx = acc.len() - 1;
                acc[last_idx] = t;
            }
            (t, _) => {
                acc.push(t);
            }
        }
    }

    acc
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    ListStart,
    ListEnd,
    Comma,
    Number(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    List(RefCell<Vec<Rc<PacketData>>>),
    Number(u64),
}

impl PacketData {
    pub fn new_number(n: u64) -> Rc<PacketData> {
        Rc::new(PacketData::Number(n))
    }

    pub fn new_list(elements: Vec<Rc<PacketData>>) -> Rc<PacketData> {
        Rc::new(PacketData::List(RefCell::new(elements)))
    }

    fn is_sorted(&self, other: &Self) -> Sort {
        match (self, other) {
            (PacketData::Number(l), PacketData::Number(r)) if l < r => Sort::Sorted,
            (PacketData::Number(l), PacketData::Number(r)) if l > r => Sort::Unsorted,
            (PacketData::Number(_), PacketData::Number(_)) => Sort::Undecided,
            (PacketData::List(left_list), PacketData::List(right_list)) => {
                let left = left_list.borrow();
                let right = right_list.borrow();
                for i in 0..std::cmp::max(left.len(), right.len()) {
                    let l = left.get(i);
                    let r = right.get(i);
                    let collection_empty = match (l, r) {
                        (None, Some(_)) => Some(Sort::Sorted),
                        (Some(_), None) => Some(Sort::Unsorted),
                        _ => None,
                    };
                    if let Some(s) = collection_empty {
                        return s;
                    }

                    let decision = l.unwrap().is_sorted(&r.unwrap());

                    if decision != Sort::Undecided {
                        return decision;
                    }
                }

                return Sort::Undecided;
            }
            (PacketData::List(_), PacketData::Number(n)) => self.is_sorted(&PacketData::List(
                RefCell::new(vec![Rc::new(PacketData::Number(*n))]),
            )),
            (PacketData::Number(n), PacketData::List(_)) => {
                PacketData::List(RefCell::new(vec![Rc::new(PacketData::Number(*n))]))
                    .is_sorted(other)
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.is_sorted(other) {
            Sort::Undecided => std::cmp::Ordering::Equal,
            Sort::Sorted => std::cmp::Ordering::Less,
            Sort::Unsorted => std::cmp::Ordering::Greater,
        }
    }
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketData::List(l) => {
                if let Err(e) = write!(f, "[") {
                    return Err(e);
                }
                for (i, e) in l.borrow().iter().enumerate() {
                    if i != 0 {
                        if let Err(e) = write!(f, ",") {
                            return Err(e);
                        }
                    }
                    if let Err(e) = e.fmt(f) {
                        return Err(e);
                    }
                }
                write!(f, "]")
            }
            PacketData::Number(n) => n.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Sort {
    Undecided,
    Sorted,
    Unsorted,
}

#[test]
fn test_sort_1() {
    let left = PacketData::new_list(wrap_numbers(vec![1, 1, 3, 1, 1]));
    let right = PacketData::new_list(wrap_numbers(vec![1, 1, 5, 1, 1]));

    let sorted = left.is_sorted(&right);

    assert_eq!(sorted, Sort::Sorted);
}

#[test]
fn test_sort_2() {
    let left = PacketData::new_list(vec![
        PacketData::new_list(wrap_numbers(vec![1])),
        PacketData::new_list(wrap_numbers(vec![2, 3, 4])),
    ]);
    let right = PacketData::List(RefCell::new(vec![
        PacketData::new_list(wrap_numbers(vec![1])),
        PacketData::new_number(4),
    ]));

    let sorted = left.is_sorted(&right);

    assert_eq!(sorted, Sort::Sorted);
}

#[test]
fn test_sort_3() {
    let left = PacketData::new_list(wrap_numbers(vec![9]));
    let right = PacketData::new_list(vec![PacketData::new_list(wrap_numbers(vec![8, 7, 6]))]);

    let sorted = left.is_sorted(&right);

    assert_eq!(sorted, Sort::Unsorted);
}

#[test]
fn test_sort_4() {
    let left = PacketData::List(RefCell::new(vec![
        PacketData::new_list(wrap_numbers(vec![4, 4])),
        PacketData::new_number(4),
        PacketData::new_number(4),
    ]));
    let right = PacketData::new_list(vec![
        PacketData::new_list(wrap_numbers(vec![4, 4])),
        PacketData::new_number(4),
        PacketData::new_number(4),
        PacketData::new_number(4),
    ]);

    let sorted = left.is_sorted(&right);

    assert_eq!(sorted, Sort::Sorted);
}

#[test]
fn test_sort_5() {
    let left = PacketData::new_list(wrap_numbers(vec![7, 7, 7, 7]));
    let right = PacketData::new_list(wrap_numbers(vec![7, 7, 7]));

    let sorted = left.is_sorted(&right);

    assert_eq!(sorted, Sort::Unsorted);
}

#[test]
fn test_sort_6() {
    let left = PacketData::new_list(vec![]);
    let right = PacketData::new_list(wrap_numbers(vec![3]));

    let sorted = left.is_sorted(&right);

    assert_eq!(sorted, Sort::Sorted);
}

#[test]
fn test_sort_7() {
    let left = PacketData::new_list(vec![PacketData::new_list(vec![PacketData::new_list(
        vec![],
    )])]);
    let right = PacketData::new_list(vec![PacketData::new_list(vec![])]);

    let sorted = left.is_sorted(&right);

    assert_eq!(sorted, Sort::Unsorted);
}

#[test]
fn test_sort_8() {
    let left = PacketData::new_list(vec![
        PacketData::new_number(1),
        PacketData::new_list(vec![
            PacketData::new_number(2),
            PacketData::new_list(vec![
                PacketData::new_number(3),
                PacketData::new_list(vec![
                    PacketData::new_number(4),
                    PacketData::new_list(wrap_numbers(vec![5, 6, 7])),
                ]),
            ]),
        ]),
        PacketData::new_number(8),
        PacketData::new_number(9),
    ]);
    let right = PacketData::new_list(vec![
        PacketData::new_number(1),
        PacketData::new_list(vec![
            PacketData::new_number(2),
            PacketData::new_list(vec![
                PacketData::new_number(3),
                PacketData::new_list(vec![
                    PacketData::new_number(4),
                    PacketData::new_list(wrap_numbers(vec![5, 6, 0])),
                ]),
            ]),
        ]),
        PacketData::new_number(8),
        PacketData::new_number(9),
    ]);

    let sorted = left.is_sorted(&right);

    assert_eq!(sorted, Sort::Unsorted);
}

#[test]
fn test_full() {
    let input = concat!(
        "[1,1,3,1,1]\n",
        "[1,1,5,1,1]\n",
        "\n",
        "[[1],[2,3,4]]\n",
        "[[1],4]\n",
        "\n",
        "[9]\n",
        "[[8,7,6]]\n",
        "\n",
        "[[4,4],4,4]\n",
        "[[4,4],4,4,4]\n",
        "\n",
        "[7,7,7,7]\n",
        "[7,7,7]\n",
        "\n",
        "[]\n",
        "[3]\n",
        "\n",
        "[[[]]]\n",
        "[[]]\n",
        "\n",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]\n",
    );

    let actual = solve_part_one(input);

    assert_eq!(actual, 13);
}

#[test]
fn test_parse_tokens() {
    let chars = "[[8],[]]".chars();

    let expected = vec![
        Token::ListStart,
        Token::ListStart,
        Token::Number(String::from("8")),
        Token::ListEnd,
        Token::Comma,
        Token::ListStart,
        Token::ListEnd,
        Token::ListEnd,
    ];

    let actual = parse_tokens(chars);

    assert_eq!(actual, expected);
}

#[test]
fn test_parse_tokens_long() {
    let chars = "[[3],[0,7,[[6,0,0,10],9,[],9,2]],[[10,3,7,[6]]]]".chars();

    let expected = vec![
        Token::ListStart,
        Token::ListStart,
        Token::Number(String::from("3")),
        Token::ListEnd,
        Token::Comma,
        Token::ListStart,
        Token::Number(String::from("0")),
        Token::Comma,
        Token::Number(String::from("7")),
        Token::Comma,
        Token::ListStart,
        Token::ListStart,
        Token::Number(String::from("6")),
        Token::Comma,
        Token::Number(String::from("0")),
        Token::Comma,
        Token::Number(String::from("0")),
        Token::Comma,
        Token::Number(String::from("10")),
        Token::ListEnd,
        Token::Comma,
        Token::Number(String::from("9")),
        Token::Comma,
        Token::ListStart,
        Token::ListEnd,
        Token::Comma,
        Token::Number(String::from("9")),
        Token::Comma,
        Token::Number(String::from("2")),
        Token::ListEnd,
        Token::ListEnd,
        Token::Comma,
        Token::ListStart,
        Token::ListStart,
        Token::Number(String::from("10")),
        Token::Comma,
        Token::Number(String::from("3")),
        Token::Comma,
        Token::Number(String::from("7")),
        Token::Comma,
        Token::ListStart,
        Token::Number(String::from("6")),
        Token::ListEnd,
        Token::ListEnd,
        Token::ListEnd,
        Token::ListEnd,
    ];

    let actual = parse_tokens(chars);

    assert_eq!(actual, expected);
}

#[test]
fn test_input() {
    let input = concat!(
        "[[8],[]]\n",
        "[[3],[0,7,[[6,0,0,10],9,[],9,2]],[[10,3,7,[6]]]]\n"
    );

    let actual = solve_part_one(input);

    assert_eq!(actual, 0);
}

#[allow(dead_code)]
fn wrap_numbers<I>(nums: I) -> Vec<Rc<PacketData>>
where
    I: IntoIterator<Item = u64>,
{
    nums.into_iter()
        .map(|n| PacketData::new_number(n))
        .collect()
}
