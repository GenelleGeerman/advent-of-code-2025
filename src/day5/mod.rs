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

fn calc(mut ranges: Vec<(u128, u128)>, ids: Vec<u128>) -> Day5Results {
    let mut output = Day5Results::new();

    for id in ids {
        if ranges.iter().any(|range| id >= range.0 && id <= range.1) {
            output.part_1_fresh_items += 1;
        };
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut filtered_ranges: Vec<(u128, u128)> = vec![];
    for range in ranges {
        if filtered_ranges.len() == 0 {
            filtered_ranges.push(range);
            continue;
        }

        let previous_range = filtered_ranges.last().unwrap();

        if range.0 > previous_range.1 {
            filtered_ranges.push(range);
            continue;
        }

        if is_in_range(*previous_range, range.0) && !is_in_range(*previous_range, range.1) {
            let index = &filtered_ranges.len() - 1;
            filtered_ranges[index] = (previous_range.0, range.1);
        }
    }

    output.part_2_fresh_items = filtered_ranges.iter().map(|range| range.1 - range.0 + 1).sum();
    return output;
}

fn is_in_range(range: (u128, u128), number: u128) -> bool {
    number >= range.0 && number <= range.1
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
    fn part_2_test_2() {
        let input = "3-5\n10-14";
        let res = day5(String::from(input));
        assert_eq!(res.part_2_fresh_items, 8);
    }

    #[test]
    fn part_2_test_3() {
        let input = "5-10\n10-14";
        let res = day5(String::from(input));
        assert_eq!(res.part_2_fresh_items, 10);
    }

    #[test]
    fn part_2_test_4() {
        let input = "3-5\n5-10\n2-7";
        let res = day5(String::from(input));
        assert_eq!(res.part_2_fresh_items, 9);
    }

    #[test]
    fn part_2_test_5() {
        let input = "0-10\n5-9\n6-8";
        let res = day5(String::from(input));
        assert_eq!(res.part_2_fresh_items, 11);
    }

    #[test]
    fn part_2_test_solution() {
        let input = fs::read_to_string("src/day5/input.txt").unwrap();
        let res = day5(String::from(input));
        assert_eq!(res.part_2_fresh_items, 334714395325710);
    }
}
