
use std::fs;
use std::iter::Sum;
use std::cmp;

#[derive(Debug)]
struct Box {
    red: u32,
    blue: u32,
    green: u32,
}

struct Reach {
    red: u32,
    blue: u32,
    green: u32,
}

impl Default for Box {
    fn default() -> Box {
        Box {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Sum for Box {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = Box { red: 0, green: 0, blue: 0};
        for b in iter {
            total.red += b.red;
            total.green += b.green;
            total.blue += b.blue;
        }
        return total;
    }
}

impl Default for Reach {
    fn default() -> Reach {
        Reach {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Sum for Reach {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = Reach { red: 0, green: 0, blue: 0};
        for b in iter {
            total.red = cmp::max(total.red, b.red);
            total.green = cmp::max(total.green, b.green);
            total.blue = cmp::max(total.blue, b.blue);
        }
        return total;
    }
}

fn decode(st: &str) -> Box {
    // println!("\t\tdecoding: {st}");
    let b = match 
        match st.trim().to_owned() {
            s if s.ends_with("red") => {
                let num = &s[..(s.len()-3)].trim();
                // println!("\t\t\tparsung num: !{num}!");
                Some(Box {red: num.parse().unwrap(), ..Default::default()})
            }
            s if s.ends_with("green") => {
                let num = &s[..(s.len()-5)].trim();
                // println!("\t\t\tparsung num: !{num}!");
                Some(Box {green: num.parse().unwrap(), ..Default::default()})
            }
            s if s.ends_with("blue") => {
                let num = &s[..(s.len()-4)].trim();
                // println!("\t\t\tparsung num: !{num}!");
                Some(Box {blue: num.parse().unwrap(), ..Default::default()})
            }
            _ => None,
        } {
            Some(b) => b,
            None => Box {..Default::default()},
        };
    // println!("\t\tdecoded: {:?}", b);
    return b;
}

fn parse_single_throw(s: &str) -> Reach {
    // println!("\tparsing single throw: {s}");
    let b: Box = s.split(",").map(decode).sum();
    let r = Reach {red: b.red, blue: b.blue, green: b.green};
    return r;
}

fn parse_line(s: &str) -> u32 {
    println!("parsing line: {s}");
    let head_and_rest: Vec<&str> = s.split(":").collect();
    let sum_boxes: Reach = head_and_rest[1].split(";").map(parse_single_throw).sum();
    let game_id: u32 = if sum_boxes.red > 12 || sum_boxes.green > 13 || sum_boxes.blue > 14 {
        0
    } else {
        head_and_rest[0].trim().strip_prefix("Game").unwrap().trim().parse().unwrap()
    };
    
    return game_id;
    
}

fn main() {
    let contents = fs::read_to_string("input_files/day_2/part1.txt").expect("No such file found!");
    let number:u32 = contents.lines().map(parse_line).sum();
    println!("The result is: {number}");
}
