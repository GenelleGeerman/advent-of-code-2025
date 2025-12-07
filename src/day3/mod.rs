pub struct Day3Results {
    pub part_1_joltage: u128,
    pub part_2_joltage: u128,
}

impl Day3Results {
    fn new() -> Self {
        Self {
            part_1_joltage: 0,
            part_2_joltage: 0,
        }
    }
}

pub fn day3(input: String) -> Day3Results {
    let mut output = Day3Results::new();
    for lines in input.lines() {
        let res = calc(lines);
        output.part_1_joltage += res.part_1_joltage;
        output.part_2_joltage += res.part_2_joltage;
    }

    return output;
}
fn calc(input: &str) -> Day3Results {
    if input.len() < 12 {
        panic!("Input is smaller than 12!")
    }

    let mut output = Day3Results::new();
    let mut x: u128 = 0;
    let mut y: u128 = 0;
    let char_array: Vec<char> = input.chars().collect();
    for i in 0..input.len() as i32 {
        let ch: char = *char_array.get(i as usize).unwrap();
        let digit: u128 = ch.to_digit(10).unwrap().into();

        //part 1
        if digit > x && i < input.len() as i32 - 1 {
            x = digit;
            y = 0;
        } else if digit > y {
            y = digit;
        }
    }
    output.part_1_joltage = (x * 10) + y;

    let mut part_2_char_array: Vec<char> = input.chars().collect();
    for i in (0..12).rev() {
        let mut largest_value = 0;
        let mut largest_index = 0;
        let (left, _) = part_2_char_array.split_at(part_2_char_array.len() - i);
        for j in 0..left.len() {
            let digit = left.get(j).unwrap().to_digit(10).unwrap();
            if digit > largest_value {
                largest_value = digit;
                largest_index = j;
            }
        }
        output.part_2_joltage += (largest_value as u128) * 10u128.pow(i as u32);
        part_2_char_array = part_2_char_array.split_at(largest_index + 1).1.to_vec();
    }
    return output;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_test_1() {
        let res = day3(String::from("987654321111111"));
        assert_eq!(res.part_1_joltage, 98);
    }

    #[test]
    fn part_1_test_2() {
        let res = day3(String::from("811111111111119"));
        assert_eq!(res.part_1_joltage, 89);
    }

    #[test]
    fn part_1_test_3() {
        let res = day3(String::from("234234234234278"));
        assert_eq!(res.part_1_joltage, 78);
    }

    #[test]
    fn part_1_test_4() {
        let res = day3(String::from("818181911112111"));
        assert_eq!(res.part_1_joltage, 92);
    }

    #[test]
    fn part_1_test_solution() {
        let day3_input = fs::read_to_string("src/day3/input.txt").unwrap();
        let res = day3(String::from(day3_input));
        assert_eq!(res.part_1_joltage, 17281);
    }

    #[test]
    fn part_2_test_1() {
        let res = day3(String::from("987654321111111"));
        assert_eq!(res.part_2_joltage, 987654321111);
    }

    #[test]
    fn part_2_test_2() {
        let res = day3(String::from("811111111111119"));
        assert_eq!(res.part_2_joltage, 811111111119);
    }

    #[test]
    fn part_2_test_3() {
        let res = day3(String::from("234234234234278"));
        assert_eq!(res.part_2_joltage, 434234234278);
    }

    #[test]
    fn part_2_test_4() {
        let res = day3(String::from("818181911112111"));
        assert_eq!(res.part_2_joltage, 888911112111);
    }

    #[test]
    fn part_2_test_5() {
        let res = day3(String::from("987654321111111\n811111111111119\n234234234234278\n818181911112111"));
        assert_eq!(res.part_2_joltage, 3121910778619);
    }

    #[test]
    fn part_2_test_6() {
        let res = day3(String::from(
            "3722443164324852429541739322454443622742537425744313396455466849784737627295682866595242427454396354",
        ));
        assert_eq!(res.part_2_joltage, 999999796354);
    }

    #[test]
    fn part_2_test_solution() {
        let day3_input = fs::read_to_string("src/day3/input.txt").unwrap();
        let res = day3(String::from(day3_input));
        assert_eq!(res.part_2_joltage, 171388730430281);
    }
}
