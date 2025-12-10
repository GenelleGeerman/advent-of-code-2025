use std::{
    collections::{HashMap, HashSet},
    u128,
};

pub struct Day8Results {
    pub part_1_total_splits: i32,
    pub part_2_total_splits: u128,
}
impl Day8Results {
    fn new() -> Self {
        Self {
            part_1_total_splits: 0,
            part_2_total_splits: 0,
        }
    }
}

pub fn day8(input: String) -> Day8Results {
    let s = input.find("S").unwrap();
    return calc();
}

fn calc() -> Day8Results {
    let output = Day8Results::new();

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_1() {
        let input = fs::read_to_string("src/day8/test.txt").unwrap();
        let res = day8(input);
        assert_eq!(res.part_1_total_splits, 21);
    }

    #[test]
    fn part_1_test_solution() {
        let input = fs::read_to_string("src/day8/input.txt").unwrap();
        let res = day8(input);
        assert_eq!(res.part_1_total_splits, 1585);
    }

    #[test]
    fn part_2_test_1() {
        let input = fs::read_to_string("src/day8/test.txt").unwrap();
        let res = day8(input);
        assert_eq!(res.part_2_total_splits, 40);
    }

    #[test]
    fn part_2_test_solution() {
        let input = fs::read_to_string("src/day8/input.txt").unwrap();
        let res = day8(input);
        assert_eq!(res.part_2_total_splits, 16716444407407);
    }
}
