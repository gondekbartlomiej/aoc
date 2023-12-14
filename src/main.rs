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

fn check_nodez(nodes: &Vec<String>) -> bool {
    nodes.iter().map(|n| n.ends_with("Z")).all(|x| x)
}

fn make_step(dir: char, map: &HashMap<String, (String, String)>, curr_nodes: Vec<String>) -> Vec<String> {
    
    let new_nodes: Vec<String> = match dir {
        'L' => curr_nodes.iter().map(|n| map[n].0.clone()).collect(),
        _ => curr_nodes.iter().map(|n| map[n].1.clone()).collect(),
    };
    new_nodes
    
}

fn main() {
    let contents = fs::read_to_string("input_files/day_8/part1.txt").expect("No such file found!");
    let path: String = contents.lines().next().unwrap().trim().to_owned();
    let map: HashMap<String, (String, String)> = contents.lines().filter_map(parse_map_node).collect();
    let mut nodes: Vec<String> = map.keys().filter(|n| n.ends_with("A")).cloned().collect();
    let mut path_iter = path.chars().cycle();
    let mut number: u32 = 0;
    while !check_nodez(&nodes) {
        number += 1;
        nodes = make_step(path_iter.next().unwrap(), &map, nodes);
    }
    println!("The result is: {number}");
}
