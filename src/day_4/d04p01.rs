use std::fs;
use std::collections::HashSet;

fn parse_line(line: &str) -> u32 {
    match line.split_once(":") {
        None => 0,
        Some((_card_num, numbers)) => {
            match numbers.split_once("|") {
                None => 0,
                Some((expected, actual)) => {
                    let expected_set: HashSet<u32> = HashSet::from_iter(expected.split(" ").filter_map(|s| s.parse::<u32>().ok()));
                    let actual_set: HashSet<u32> = HashSet::from_iter(actual.split(" ").filter_map(|s| s.parse::<u32>().ok()));
                    
                    match expected_set.intersection(&actual_set).count() {
                        n if n > 0 => u32::pow(2, u32::try_from(n).unwrap()-1),
                        _ => 0
                    } 
                }
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input_files/day_4/part1.txt").expect("No such file found!");
    let number:u32 = contents.lines().map(parse_line).sum();
    println!("The result is: {number}");
}
