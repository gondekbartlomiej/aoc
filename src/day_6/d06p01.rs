use std::collections::HashMap;
use std::fs;

fn parse_line(_s: &str) -> u32 {
    0
}

fn parse_to_numbers(s: &str) -> Vec<u32> {
    s.split(" ")
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

fn find_num_of_better_results(t: u32, r: u32) -> i64 {

    let t_f64 = f64::from(t);
    let r_f64 = f64::from(r);

    let fmax = t_f64.powi(2) / 4_f64;
    let dist = (fmax - r_f64 - 0.8).sqrt();

    let num = dist.ceil() as i64;

    let num_of_better = match t % 2 == 0 {
        true => num * 2 - 1,
        false => num * 2,
    };
    num_of_better
}

fn main() {
    let contents = fs::read_to_string("input_files/day_6/part1.txt").expect("No such file found!");

    let map = contents
        .lines()
        .filter_map(|l| l.split_once(":"))
        .map(|(h, t)| (h.to_owned(), parse_to_numbers(t)))
        .collect::<HashMap<String, Vec<u32>>>();

    let tr_tuples: Vec<(u32, u32)> =
        map["Time"].iter()
        .zip(map["Distance"].iter())
        .map(|(t, r)| (t.clone(), r.clone()))
        .collect();

    let number: i64 = tr_tuples.iter().fold(1, |acc, (t,r)| acc*find_num_of_better_results(t.clone(), r.clone()));

    println!("Number is {number}");
}

