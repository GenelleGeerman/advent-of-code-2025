use std::fs;

mod day1;
mod day2;
mod day3;

fn main() {
    //day 1
    println!("-------------------------------------------------");
    println!("Day 1");
    let day1_input = fs::read_to_string("src/day1/input.txt").unwrap();
    let day1_results = day1::day1(day1_input);
    println!("part 1 Password: {}", day1_results.part_1_password);
    println!("part 2 Password: {}", day1_results.part_2_password);

    println!("-------------------------------------------------");
    println!("Day 2");
    let day2_input = fs::read_to_string("src/day2/input.txt").unwrap();
    let day2_results = day2::day2(String::from(day2_input));
    println!("part 1 Password: {}", day2_results.part_1_total);
    println!("part 2 Password: {}", day2_results.part_2_total);
    println!("-------------------------------------------------");
    println!("Day 3");
    let day3_input = fs::read_to_string("src/day3/input.txt").unwrap();
    let day3_results = day3::day3(day3_input);
    println!("part 1 Password: {}", day3_results.part_1_joltage);
    println!("part 2 Password: {}", day3_results.part_2_joltage);
}
