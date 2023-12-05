use std::fs;
use std::collections::HashSet;
use std::cmp;

fn parse_line(line: &str) -> u32 {
    match line.split_once(":") {
        None => 0,
        Some((_card_num, numbers)) => {
            match numbers.split_once("|") {
                None => 0,
                Some((expected, actual)) => {
                    let expected_set: HashSet<u32> = HashSet::from_iter(expected.split(" ").filter_map(|s| s.parse::<u32>().ok()));
                    let actual_set: HashSet<u32> = HashSet::from_iter(actual.split(" ").filter_map(|s| s.parse::<u32>().ok()));
                    
                    u32::try_from(expected_set.intersection(&actual_set).count()).unwrap()
                     
                }
            }
        }
    }
}

fn get_sum_scratchcards_rooting_from(id: usize, v: &[u32]) -> u32 {
    // if v.len() == 0 {
    //     return 0;
    // }
    match v[id] {
        0 => 1,
        n => {
            let _from = usize::try_from(id+1).unwrap();
            // let _to = _from + usize::try_from(n).unwrap();
            let from = cmp::min(v.len(), _from);
            // let to = cmp::min(v.len(), _to);
            let subset:&[u32] = &v[from..];
            let num = usize::try_from(n).unwrap();
            let sum_rooting:u32 = subset.iter().take(num).enumerate().map(|(id, _)| get_sum_scratchcards_rooting_from(id, subset)).sum();
            1+sum_rooting
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input_files/day_4/part1.txt").expect("No such file found!");
    let points:Vec<u32> = contents.lines().map(parse_line).collect();
    println!("The matches are: {:?}", points);
    let sum_scratchcards:u32 = points.iter().enumerate().map(|(id, _)| get_sum_scratchcards_rooting_from(id, &points)).sum();

    println!("The result is: {sum_scratchcards}");
}
