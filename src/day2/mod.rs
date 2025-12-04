use std::fmt::Error;

pub struct Day2Result {
    pub part_1_total: u128,
    pub part_2_total: u128,
}

impl Day2Result {
    fn new() -> Self {
        Self { part_1_total: 0, part_2_total: 0 }
    }
}
pub fn day2(input: String) -> Day2Result {
    let mut output = Day2Result::new();
    let ranges: Vec<&str> = input.split(",").collect();

    for range in ranges {
        let (start, end): (u128, u128) = range.split_once("-").map(|t| (t.0.parse().unwrap(), t.1.parse().unwrap())).unwrap();
        output.part_1_total += calc_1(start, end);
        output.part_2_total += calc_2(start, end);
    }
    return output;
}

fn calc_1(start: u128, end: u128) -> u128 {
    let mut output = 0;

    for digit in start..=end {
        let d_string = digit.to_string();
        if (d_string.len() % 2) == 1 {
            continue;
        }
        let (part_1, part_2) = d_string.split_at(d_string.len() / 2);
        if part_1 == part_2 {
            output += digit;
        }
    }
    return output;
}

fn calc_2(start: u128, end: u128) -> u128 {
    let mut output = 0;

    'outer: for digit in start..=end {
        let d_string = digit.to_string();
        let d_chars: Vec<char> = digit.to_string().chars().collect();
        let mut loopy: String = "".to_owned();
        for i in 0..d_string.len() {
            loopy = loopy + &d_chars[i].to_string();
            let test = d_string.split_at(loopy.len());
            if test.0 == test.1 {
                output += digit;
                continue 'outer;
            }
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_1() {
        let res = day2(String::from("11-22"));
        assert_eq!(res.part_1_total, 33);
    }

    #[test]
    fn part_1_test_2() {
        let res = day2(String::from("95-115"));
        assert_eq!(res.part_1_total, 99);
    }

    #[test]
    fn part_1_test_3() {
        let res = day2(String::from("998-1012"));
        assert_eq!(res.part_1_total, 1010);
    }

    #[test]
    fn part_1_test_4() {
        let res = day2(String::from("1188511880-1188511890"));
        assert_eq!(res.part_1_total, 1188511885);
    }

    #[test]
    fn part_1_test_5() {
        let res = day2(String::from("222220-222224"));
        assert_eq!(res.part_1_total, 222222);
    }

    #[test]
    fn part_1_test_6() {
        let res = day2(String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        ));
        assert_eq!(res.part_1_total, 1227775554);
    }
    #[test]
    fn part_1_test_solution() {
        let day2_input = fs::read_to_string("src/day2/input.txt").unwrap();
        let res = day2(day2_input);
        assert_eq!(res.part_1_total, 30599400849);
    }

    #[test]
    fn part_2_test_1() {
        let res = day2(String::from("11-22"));
        assert_eq!(res.part_2_total, 33);
    }

    #[test]
    fn part_2_test_2() {
        let res = day2(String::from("95-115"));
        assert_eq!(res.part_2_total, 210);
    }

    #[test]
    fn part_2_test_3() {
        let res = day2(String::from("998-1012"));
        assert_eq!(res.part_2_total, 2009);
    }

    #[test]
    fn part_2_test_4() {
        let res = day2(String::from("1188511880-1188511890"));
        assert_eq!(res.part_2_total, 1188511885);
    }

    #[test]
    fn part_2_test_5() {
        let res = day2(String::from("222220-222224"));
        assert_eq!(res.part_2_total, 222222);
    }

    #[test]
    fn part_2_test_6() {
        let res = day2(String::from(
            "11-22,95-115,99-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        ));
        assert_eq!(res.part_2_total, 4174379265);
    }

    //#[test]
    fn part_2_test_solution() {
        let day2_input = fs::read_to_string("src/day2/input.txt").unwrap();
        let res = day2(day2_input);
        assert_eq!(res.part_2_total, 30599400849);
    }
}
