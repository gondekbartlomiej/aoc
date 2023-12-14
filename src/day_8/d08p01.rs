use std::fs;
use std::collections::HashMap;

fn parse_line(_s: &str) -> u32 {
    0
}

fn parse_map_node(s: &str) -> Option<(String, (String, String))> {
    match s.split_once("="){
        Some((key, vals)) => {
             let vals_split = match vals
                    .replace(|c| (c == '(') || (c == ')'), "")
                    .trim()
                    .split_once(", ") {
                        Some((l, r)) => (l.to_owned(), r.to_owned()),
                        None => {return None},
                    };
            Some((key.trim().to_owned(), vals_split))
        },
        None => None,
    }
}

fn get_steps(path: &str, path_left: &str, map: &HashMap<String, (String, String)>, curr_node: &str, curr_steps: u32) -> u32 {
    
    match curr_node.eq("ZZZ") {
        true => curr_steps,
        false => {
            let new_path_left = match path_left.len() {
                1 => path,
                _ => &path_left[1..],
            };
            match path_left.starts_with("L") {
                true => get_steps(path, new_path_left, map, &map[curr_node].0[..], curr_steps+1),
                false => get_steps(path, new_path_left, map, &map[curr_node].1[..], curr_steps+1),
            }
        },
    }
    
}

fn main() {
    let contents = fs::read_to_string("input_files/day_8/part1.txt").expect("No such file found!");
    let path: String = contents.lines().next().unwrap().trim().to_owned();
    let map: HashMap<String, (String, String)> = contents.lines().filter_map(parse_map_node).collect();
    let number = get_steps(&path[..], &path[..], &map, "AAA", 0);
    println!("The result is: {number}");
}
