pub struct Day3Results {
    pub part_1_joltage: u32,
    pub part_2_joltage: u32,
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
    let mut output = Day3Results::new();
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut i: i32 = 0;
    for c in input.chars() {
        let digit: u32 = c.to_digit(10).unwrap();
        i += 1;
        if digit > x && i < input.len().try_into().unwrap() {
            x = digit;
            y = 0;
        } else if digit > y {
            y = digit;
        }
    }

    output.part_1_joltage = (x * 10) + y;

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
}
