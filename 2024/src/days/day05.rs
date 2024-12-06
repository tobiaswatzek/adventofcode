use std::collections::{HashMap, HashSet};
use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> usize {
    let (rules_list, print_orders) = parse_input(input);

    let rules = rules_list.iter().fold(HashMap::new(), |mut acc, (a, b)| {
        if !acc.contains_key(b) {
            acc.insert(*b, HashSet::new());
        }

        acc.get_mut(b).unwrap().insert(*a);

        acc
    });

    let mut middle_numbers = vec![];

    for print_order in print_orders.iter() {
        let page_idx_map: HashMap<usize, usize> = print_order
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i))
            .collect();

        let is_ordered = print_order.iter().enumerate().all(|(i, page)| {
            let page_rules_opt = rules.get(&page);
            if page_rules_opt.is_none() {
                return true;
            }
            let page_rules = page_rules_opt.unwrap();

            page_rules
                .iter()
                .filter(|prev_page| page_idx_map.contains_key(prev_page))
                .all(|prev_page| *page_idx_map.get(prev_page).unwrap() < i)
        });

        if !is_ordered {
            continue;
        }

        middle_numbers.push(print_order[print_order.len() / 2]);
    }
    
    middle_numbers.iter().sum()
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut rules = vec![];
    let mut print_orders = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some((aStr, bStr)) = line.split_once("|") {
            let before = aStr.parse::<usize>().unwrap();
            let after = bStr.parse::<usize>().unwrap();
            rules.push((before, after));
            continue;
        }

        let order = line
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if order.len() > 0 {
            print_orders.push(order);
        }
    }

    (rules, print_orders)
}

fn solve_second(_input: &str) -> i64 {
    0
}
