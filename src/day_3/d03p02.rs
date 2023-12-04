
use std::fs;
use std::cmp;

fn find_adjusting_numbers(s: &str, pos: u32) -> Vec<u32> {
    let candidates: Vec<&str> = s.split(&['.', '*'][..]).filter(|&s| !s.is_empty()).collect();
    if candidates.is_empty() {
        return Vec::new();
    } else {

        let candidate = candidates[0];
        let cand_pos = i64::try_from(s.find(candidate).unwrap()).unwrap();
        let cand_len = i64::try_from(candidate.len()).unwrap();

        let dist_front:i64 = i64::from(pos) - cand_pos; 
        let dist_back:i64 = i64::from(pos) - (cand_pos + cand_len - 1); 

        let new = s.replacen(candidate, &".".repeat(candidate.len()), 1);
        
        let mut vals:Vec<u32> = find_adjusting_numbers(&new[..], pos);
        let overlaps: bool = (cand_pos <= i64::from(pos)) && (cand_pos + cand_len >= i64::from(pos));
        if (dist_front.abs() <= 1) || (dist_back.abs() <= 1) || overlaps {
            if let Ok(val) = candidate.parse() {
                vals.push(val);
            }
        }
        return vals;
    }
}

fn get_sum_of_gear_ratios_from_line(prev: &str, curr: &str, next: &str) -> u32 {
    return match curr.find('*') {
        None => 0,
        Some(_pos) => {
            // let mut pos:i32 = i32::try_from(_pos).unwrap();
            // pos = cmp::max(0, pos-3);

            // let from = usize::try_from(pos).unwrap();

            // let p_from: usize = cmp::min(prev.len(), from);
            // let c_from: usize = cmp::min(curr.len(), from);
            // let n_from: usize = cmp::min(next.len(), from);
            //
            // let p_to: usize = cmp::min(prev.len(), _pos+4);
            // let c_to: usize = cmp::min(curr.len(), _pos+4);
            // let n_to: usize = cmp::min(next.len(), _pos+4);

            // let top_neighbourhood = &prev[p_from..p_to];
            // let cur_neighbourhood = &curr[c_from..c_to];
            // let bot_neighbourhood = &next[n_from..n_to];

            let pos = u32::try_from(_pos).unwrap();
            //println!("\tFound * at pos: {pos}");
            //println!("");
            //println!("\t{top_neighbourhood}");
            //println!("\t{cur_neighbourhood}");
            //println!("\t{bot_neighbourhood}");
            //println!("");

            let mut vals = find_adjusting_numbers(prev, pos);
            vals.append(&mut find_adjusting_numbers(next, pos));

            let left_neigh = &curr[.._pos];
            let start = cmp::min(curr.len(), _pos+1);
            let right_neigh = &curr[start..];
            //println!("\tright neigh: !{right_neigh}!");
            //println!("");

            match right_neigh.split_once(&['.', '*'][..]) {
                Some((head, _)) => {
                    //println!("\thead: !{head}!");
                    //println!("\trest: !{rest}!");
                    //println!("");
                    if let Ok(val) = head.parse::<u32>() {
                        vals.push(val);
                    }
                },
                None => {
                    //println!("\trest: !{right_neigh}!");
                    //println!("");
                    if let Ok(val) = right_neigh.parse::<u32>() {
                        vals.push(val);
                    }
                }
            }

            match left_neigh.rsplit_once(&['.', '*'][..]) {
                Some((_, rest)) => {
                    if let Ok(val) = rest.parse::<u32>() {
                        vals.push(val);
                    }
                },
                None => {
                    if let Ok(val) = left_neigh.parse::<u32>() {
                        vals.push(val);
                    }
                }
            }
            // let len = vals.len();
            //println!("\tvals len {len}");

            let this = match vals.len() {
                2 => vals[0]*vals[1],
                _ => 0,
            };
             
            if this != 0 {
                //println!("\t!!! OK !!!");
                //println!("");
                //println!("");
            }

            let new = curr.replacen('*', ".", 1);
            let rest = get_sum_of_gear_ratios_from_line(prev, &new[..], next);
            this+rest
        }
    
    };
}
    
fn main() {
    let contents = fs::read_to_string("input_files/day_3/part1.txt").expect("No such file found!");

    let mut lines = contents.lines();
    let mut prev = String::from("");
    let mut curr = String::from("");
    let mut sum:u32 = 0;


    while let Some(line) = lines.next() {
        let next = line.replace(|c| !char::is_numeric(c) && c != '*', ".");
        //println!("parsing lines:");
        //println!("{prev}");
        //println!("{curr}");
        //println!("{next}");
        //println!("");
        
        let line_sum = get_sum_of_gear_ratios_from_line(&prev[..], &curr[..], &next[..]);
        //println!("line sum is: {line_sum}");
        
        sum += line_sum;
        prev = curr;
        curr = next;
    }

    println!("The result is: {sum}");
}
