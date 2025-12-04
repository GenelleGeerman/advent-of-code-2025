use std::fs;

mod day1;
mod day2;

fn main() {
    //day 1
    println!("-------------------------------------------------");
    println!("Day 1");
    let day1_input = fs::read_to_string("src/day1/input.txt").unwrap();
    let day1_results = day1::day1(day1_input);
    println!("part 1 Password: {}", day1_results.part_1_password);
    println!("part 2 Password: {}", day1_results.part_2_password);
}
