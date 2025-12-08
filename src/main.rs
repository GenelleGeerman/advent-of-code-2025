use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    //run_day_1();
    // run_day_2();
    // run_day_3();
    //  run_day_4();
    run_day_5();
}

fn run_day_1() {
    println!("-------------------------------------------------");
    println!("Day 1");
    let day1_input = fs::read_to_string("src/day1/input.txt").unwrap();
    let day1_results = day1::day1(day1_input);
    println!("part 1 Password: {}", day1_results.part_1_password);
    println!("part 2 Password: {}", day1_results.part_2_password);
}

fn run_day_2() {
    println!("-------------------------------------------------");
    println!("Day 2");
    let day2_input = fs::read_to_string("src/day2/input.txt").unwrap();
    let day2_results = day2::day2(String::from(day2_input));
    println!("part 1 Password: {}", day2_results.part_1_total);
    println!("part 2 Password: {}", day2_results.part_2_total);
}

fn run_day_3() {
    println!("-------------------------------------------------");
    println!("Day 3");
    let day3_input = fs::read_to_string("src/day3/input.txt").unwrap();
    let day3_results = day3::day3(day3_input);
    println!("part 1 Password: {}", day3_results.part_1_joltage);
    println!("part 2 Password: {}", day3_results.part_2_joltage);
}

fn run_day_4() {
    println!("-------------------------------------------------");
    println!("Day 4");
    let day4_input = fs::read_to_string("src/day4/input.txt").unwrap();
    let day4_results = day4::day4(String::from(day4_input));
    println!("part 1 Password: {}", day4_results.part_1_rolls);
    println!("part 2 Password: {}", day4_results.part_2_rolls);
}

fn run_day_5() {
    println!("-------------------------------------------------");
    println!("Day 5");
    let day5_input = fs::read_to_string("src/day5/input.txt").unwrap();
    let day5_test = fs::read_to_string("src/day5/test.txt").unwrap();
    let day5_results = day5::day5(String::from(day5_test));
    println!("part 1 Password: {}", day5_results.part_1_fresh_items);
    println!("part 2 Password: {}", day5_results.part_2_fresh_items);
}
