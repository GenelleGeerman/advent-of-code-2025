pub struct Day7Results {
    pub part_1_total_splits: i32,
    pub part_2_total_splits: i32,
}
impl Day7Results {
    fn new() -> Self {
        Self {
            part_1_total_splits: 0,
            part_2_total_splits: 0,
        }
    }
}

pub fn day7(input: String) -> Day7Results {
    let s = input.find("S").unwrap();
    return calc(s, input);
}
fn calc(s: usize, input: String) -> Day7Results {
    let output = Day7Results::new();
    let last_index: Vec<usize> = vec![];
    let current_index: Vec<usize> = vec![];
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_1() {
        let input = fs::read_to_string("src/day7/test.txt").unwrap();
        let res = day7(input);
        assert_eq!(res.part_1_total_splits, 21);
    }
}
