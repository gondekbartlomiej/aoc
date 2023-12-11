use std::collections::HashMap;
use std::fs;

fn parse_line(_s: &str) -> u32 {
    0
}

fn parse_to_numbers(s: &str) -> i64 {
    s.replace(" ", "")
        .parse::<i64>()
        .unwrap()
}

fn find_num_of_better_results(t: i64, r: i64) -> i64 {

    let t_f64 = t as f64;
    let r_f64 = r as f64;
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
    let contents = fs::read_to_string("input_files/day_6/ex1.txt").expect("No such file found!");

    let map = contents
        .lines()
        .filter_map(|l| l.split_once(":"))
        .map(|(h, t)| (h.to_owned(), parse_to_numbers(t)))
        .collect::<HashMap<String, i64>>();

    let number: i64 = find_num_of_better_results(map["Time"], map["Distance"]);

    println!("Number is {number}");
}

