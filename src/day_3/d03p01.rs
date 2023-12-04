use std::fs;
use std::cmp;
use std::collections::HashSet;

fn contains_symbol(s: &str, symbols: &HashSet<char>) -> bool {
    // println!("\t\tlooking for symbols {:?} in {s}", symbols);
    s.chars().map(|c| symbols.contains(&c)).any(|b| b)
}

fn get_sum_of_part_numbers_from_line(prev: &str, curr: &str, next: &str, symbols: &HashSet<char>) -> u32 {
    let mut extended_symbols = symbols.clone();
    extended_symbols.insert('.');
    // println!("\tusing separators: {:?}", extended_symbols);
    let number_candidates: Vec<&str> = curr.split(|c| extended_symbols.contains(&c)).filter(|&s| !s.is_empty()).collect();
    if number_candidates.is_empty() {
        return 0;
    }
    else {
        let candidate = number_candidates[0];
        let pos = curr.find(candidate).unwrap();
        let _new = curr.replacen(candidate, &".".repeat(candidate.len()), 1);
        let _num: Result<u32, _> = number_candidates[0].parse();
        let from = match pos > 0 {
            true => pos-1,
            false => pos,
        };
        
        let p_from: usize = cmp::min(prev.len(), from);
        let c_from: usize = cmp::min(curr.len(), from);
        let n_from: usize = cmp::min(next.len(), from);

        let p_to: usize = cmp::min(prev.len(), pos+candidate.len()+1);
        let c_to: usize = cmp::min(curr.len(), pos+candidate.len()+1);
        let n_to: usize = cmp::min(next.len(), pos+candidate.len()+1);

        let top_neighbourhood = &prev[p_from..p_to];
        let cur_neighbourhood = &curr[c_from..c_to];
        let bot_neighbourhood = &next[n_from..n_to];

        println!("\tfound candidate: \"{candidate}\"");
        println!("\tZZZZZZZZ");
        println!("\tZ{top_neighbourhood}Z");
        println!("\tZ{cur_neighbourhood}Z");
        println!("\tZ{bot_neighbourhood}Z");
        println!("\tZZZZZZZZ");

        let num = match _num {
            Ok(i) if contains_symbol(top_neighbourhood, symbols) => i,
            Ok(i) if contains_symbol(cur_neighbourhood, symbols) => i,
            Ok(i) if contains_symbol(bot_neighbourhood, symbols) => i,
            _ =>  0,
        };
        println!("\tparsed to: {num}");
        println!("");
        return num + get_sum_of_part_numbers_from_line(&prev[p_to-1..], &_new[c_to-1..], &next[n_to-1..], symbols);
    }
}

fn line_into_uniq_symbols(s: &str) -> HashSet<char> {
    s.chars().filter(|c| !c.is_digit(10)).filter(|c| c != &'.').collect::<HashSet<char>>().into_iter().collect()
}

fn main() {
    let contents = fs::read_to_string("input_files/day_3/part1.txt").expect("No such file found!");

    let mut symbols: HashSet<char> = HashSet::new(); 

    let mut lines = contents.lines();
    let mut prev: &str = "";
    let mut curr: &str = "";
    let mut sum:u32 = 0;


    while let Some(next) = lines.next() {
        symbols.extend(line_into_uniq_symbols(next));

        println!("parsing lines:");
        println!("{prev}");
        println!("{curr}");
        println!("{next}");
        println!("");
        
        let line_sum = get_sum_of_part_numbers_from_line(prev, curr, next, &symbols);
        println!("line sum is: {line_sum}");
        
        sum += line_sum;
        prev = curr;
        curr = next;
    }

    println!("The symbols are: {:?}", symbols);
    println!("The result is: {sum}");
}
