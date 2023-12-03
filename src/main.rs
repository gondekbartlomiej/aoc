use std::fs;

fn main() {
    let contents = fs::read_to_string("input_files/day_x/partx.txt").expect("No such file found!");
    let number:u32 = contents.lines().map(parse_line).sum();
    println!("The result is: {number}");
}
