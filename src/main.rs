use std::fs;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn get_first_rule_rank(s: &str) -> u64 {
    let cards = s.trim()
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
        .values()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    cards.values

    0
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
    let (cards, bid) = s.split_once(" ").unwrap();
    (0, 0)
}

fn main() {
    let contents = fs::read_to_string("input_files/day_x/partx.txt").expect("No such file found!");
    // let number:u32 = contents.lines().map(parse_line).sum();
    // println!("The result is: {number}");
}
