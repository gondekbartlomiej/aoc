use std::fs;

fn get_first_digit(line: &str) -> u32 {
    let mut chrs = line.chars();
    while let Some(c) = chrs.next() {
        match c.to_digit(10) {
            Some(d) => return d,
            _ => continue,
        }
    }
    return 0;
}

fn get_last_digit(line: &str) -> u32 {
    let mut chrs = line.chars().rev();
    while let Some(c) = chrs.next() {
        match c.to_digit(10) {
            Some(d) => return d,
            _ => continue,
        }
    }
    return 0;
}

fn get_number_from_line(l: &str) -> u32 {
    let number = 10 * get_first_digit(l) + get_last_digit(l);
    println!("For line \"{l}\" number is {number}");
    return number;
}

fn main() {
    let contents = fs::read_to_string("input_files/day_1/part1.txt").expect("No such file found!");

    let number:u32 = contents.lines().map(get_number_from_line).sum();

    println!("The result is: {number}");
}

