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

    'outer: for id in ids {
        for range in &ranges {
            if id >= range.0 && id <= range.1 {
                output.part_1_fresh_items += 1;
                continue 'outer;
            }
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    println!("Sorted:");
    for range in &ranges {
        println!("{}, {}", range.0, range.1);
    }
    let mut new_ranges: Vec<(u128, u128)> = vec![];
    let mut write_range = *ranges.get(0).unwrap();
    for i in 1..ranges.len() {
        let (curr_left, curr_right) = ranges.get(i - 1).unwrap();
        let (next_left, next_right) = ranges.get(i).unwrap();
        if next_left >= curr_left && next_left <= curr_right {
            let l = curr_left;
            let r = if curr_right >= next_right { curr_right } else { next_right };

            if write_range.0 != *l {
                new_ranges.push(write_range);
            }
            write_range = (*l, *r);
        } else if next_left > curr_right {
            new_ranges.push(write_range);
            write_range = (*next_left, *next_right);
        }
    }

    let mut total: u128 = 0;

    println!("Final:");
    for range in new_ranges {
        println!("{}, {}", range.0, range.1);
        total += range.1 - range.0 + 1;
    }

    output.part_2_fresh_items = total;

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
    fn part_2_test_solution() {
        let input = fs::read_to_string("src/day5/input.txt").unwrap();
        let res = day5(String::from(input));
        assert_eq!(res.part_2_fresh_items, 848);
    }
}
