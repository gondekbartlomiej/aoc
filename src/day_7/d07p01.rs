use std::fs;
// use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn get_first_rule_rank(s: &str) -> u64 {
    let mut cards: Vec<u64> = s.trim()
        .chars()
        .fold(HashMap::new(), |mut acc: HashMap<char, u64>, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
        .values().cloned().collect();

    cards.sort_by(|a, b| b.cmp(a));

    let mut rank: u64 = cards[0];
    rank = match rank {
        3 => if cards[1] == 2 {
            rank*2
        } else {
            (rank * 2) - 1
        },
        2 => if cards[1] == 2 {
            rank*2
        } else {
            (rank * 2) - 1
        },
        1 => 0,
        _ => rank * 2,
    };
    return rank * 10000000000;
}

fn get_second_rule_rank(s: &str) -> u64 {
    s.trim()
        .replace("2", "02")
        .replace("3", "03")
        .replace("4", "04")
        .replace("5", "05")
        .replace("6", "06")
        .replace("7", "07")
        .replace("8", "08")
        .replace("9", "09")
        .replace("T", "10")
        .replace("J", "11")
        .replace("Q", "12")
        .replace("K", "13")
        .replace("A", "14")
        .parse::<u64>().unwrap()
}

fn parse_line(s: &str) -> (u64, u64) {

    let (cards, bid_str) = s.split_once(" ").unwrap();

    let first_rule_rank = get_first_rule_rank(cards);
    let second_rule_rank = get_second_rule_rank(cards);
    let rank = first_rule_rank + second_rule_rank;

    let bid = bid_str.trim().parse::<u64>().unwrap();

    (rank, bid)
}

fn main() {
    let contents = fs::read_to_string("input_files/day_7/part1.txt").expect("No such file found!");
    let mut rank_bid_v: Vec<(u64, u64)> = contents.lines().map(parse_line).collect();


    rank_bid_v.sort_by(|(a, _), (b, _)| a.cmp(b));
    let number: u64 = rank_bid_v.iter().enumerate().map(|(i, (_, bid))| ((i as u64)+1)*bid).sum();
    println!("The result is: {number}");
}
