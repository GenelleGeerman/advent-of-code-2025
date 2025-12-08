use core::num;
use std::collections::HashSet;

pub struct Day5Results {
    pub part_1_fresh_items: u128,
    pub part_2_fresh_items: u128,
}

impl Day5Results {
    fn new() -> Self {
        Self {
            part_1_fresh_items: 0,
            part_2_fresh_items: 0,
        }
    }
}

pub fn day5(input: String) -> Day5Results {
    let mut ids: Vec<u128> = vec![];
    let mut ranges: Vec<(u128, u128)> = vec![];
    for line in input.lines() {
        if line.contains("-") {
            let s: Vec<&str> = line.split("-").collect();
            ranges.push((s.get(0).unwrap().parse().unwrap(), s.get(1).unwrap().parse().unwrap()));
        } else {
            let x = line.parse();
            if x.is_ok() {
                ids.push(x.unwrap());
            }
        }
    }

    return calc(ranges, ids);
}

fn calc(ranges: Vec<(u128, u128)>, ids: Vec<u128>) -> Day5Results {
    let mut output = Day5Results::new();

    'outer: for id in ids {
        for range in &ranges {
            if id >= range.0 && id <= range.1 {
                output.part_1_fresh_items += 1;
                continue 'outer;
            }
        }
    }

    return output;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_test_1() {
        let input = fs::read_to_string("src/day5/test.txt").unwrap();
        let res = day5(String::from(input));
        assert_eq!(res.part_1_fresh_items, 3);
    }
    #[test]
    fn part_1_test_solution() {
        let input = fs::read_to_string("src/day5/input.txt").unwrap();
        let res = day5(String::from(input));
        assert_eq!(res.part_1_fresh_items, 848);
    }
    #[test]
    fn part_2_test_1() {
        let input = fs::read_to_string("src/day5/test.txt").unwrap();
        let res = day5(String::from(input));
        assert_eq!(res.part_2_fresh_items, 14);
    }
    #[test]
    fn part_2_test_solution() {
        let input = fs::read_to_string("src/day5/input.txt").unwrap();
        let res = day5(String::from(input));
        assert_eq!(res.part_2_fresh_items, 848);
    }
}
