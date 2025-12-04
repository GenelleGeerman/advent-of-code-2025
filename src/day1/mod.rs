pub struct Day1Results {
    pub dial: i32,
    pub part_1_password: i32,
    pub part_2_password: i32,
}

impl Day1Results {
    fn new() -> Self {
        Self {
            dial: 50,
            part_1_password: 0,
            part_2_password: 0,
        }
    }
}

pub fn day1(contents: String) -> Day1Results {
    let mut output = Day1Results::new();
    for line in contents.lines() {
        let split = line.split_at(1);
        let mut turns: i32 = split.1.parse().unwrap();
        if split.0.to_lowercase() == "l" {
            turns = -turns;
        }
        output = calc(turns, output);
    }
    return output;
}

fn calc(turns: i32, mut output: Day1Results) -> Day1Results {
    let mut password: i32;
    let mut dial: i32;
    let total = output.dial + turns;

    password = total / 100;
    password = password.abs();
    dial = total % 100;

    if dial < 0 {
        dial = 100 + dial;
    }

    if output.dial > 0 && total <= 0 {
        password += 1;
    }
    if dial == 0 {
        output.part_1_password += 1;
    }
    output.part_2_password += password;
    output.dial = dial;
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_solution() {
        let contents = fs::read_to_string("src/day1/input.txt").unwrap();
        let res = day1(contents);
        assert_eq!(res.part_1_password, 989);
    }

    #[test]
    fn part_2_test_1() {
        let res = day1(String::from("R50"));
        assert_eq!(res.part_2_password, 1);
        assert_eq!(res.dial, 0);
    }

    #[test]
    fn part_2_test_2() {
        let res = day1(String::from("R51"));
        assert_eq!(res.part_2_password, 1);
        assert_eq!(res.dial, 1);
    }

    #[test]
    fn part_2_test_3() {
        let res = day1(String::from("L50"));
        assert_eq!(res.part_2_password, 1);
        assert_eq!(res.dial, 0);
    }

    #[test]
    fn part_2_test_4() {
        let res = day1(String::from("L51"));
        assert_eq!(res.part_2_password, 1);
        assert_eq!(res.dial, 99);
    }

    #[test]
    fn part_2_test_6() {
        let res = day1(String::from("L51\nR100\nL201\nR3"));
        assert_eq!(res.dial, 1);
        assert_eq!(res.part_2_password, 5);
    }
    #[test]
    fn part_2_test_7() {
        let res = day1(String::from("L1000"));
        assert_eq!(res.dial, 50);
        assert_eq!(res.part_2_password, 10);
    }
    #[test]
    fn part_2_test_8() {
        let res = day1(String::from("R1000"));
        assert_eq!(res.dial, 50);
        assert_eq!(res.part_2_password, 10);
    }

    #[test]
    fn part_2_test_9() {
        let res = day1(String::from("R1000\nL1000"));
        assert_eq!(res.dial, 50);
        assert_eq!(res.part_2_password, 20);
    }

    #[test]
    fn part_2_test_example() {
        let res = day1(String::from("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82"));
        // assert_eq!(res.dial, 50);
        assert_eq!(res.part_2_password, 6);
    }

    #[test]
    fn part_2_test_solution() {
        let contents = fs::read_to_string("src/day1/input.txt").unwrap();
        let res = day1(contents);
        assert_eq!(res.part_2_password, 5941);
    }
}
