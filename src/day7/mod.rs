use std::{
    collections::{HashMap, HashSet},
    u128,
};

pub struct Day7Results {
    pub part_1_total_splits: i32,
    pub part_2_total_splits: u128,
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
    let mut output = Day7Results::new();
    let mut indices: HashSet<u128> = HashSet::new();
    let mut total_2: HashMap<u128, u128> = HashMap::new();
    indices.insert(s as u128);
    total_2.insert(s as u128, 1);
    for line in input.lines() {
        if line.chars().all(|x| x == '.') {
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        for i in 0..chars.len() as u128 {
            let c = *chars.get(i as usize).unwrap();
            if c != '^' {
                continue;
            }

            if indices.contains(&i) {
                output.part_1_total_splits += 1;
                indices.remove(&i);
                indices.insert(i - 1);
                indices.insert(i + 1);

                let val: u128 = *total_2.get(&(i as u128)).unwrap();
                println!("{i}");
                println!("{val}");
                total_2.insert(i as u128 - 1, val + total_2.get(&(i - 1)).unwrap_or(&0));
                total_2.insert(i as u128 + 1, val + total_2.get(&(i as u128 + 1)).unwrap_or(&0));
                total_2.remove(&(i as u128));
            }
        }
    }
    output.part_2_total_splits = total_2.values().sum();
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

    #[test]
    fn part_1_test_solution() {
        let input = fs::read_to_string("src/day7/input.txt").unwrap();
        let res = day7(input);
        assert_eq!(res.part_1_total_splits, 1585);
    }

    #[test]
    fn part_2_test_1() {
        let input = fs::read_to_string("src/day7/test.txt").unwrap();
        let res = day7(input);
        assert_eq!(res.part_2_total_splits, 40);
    }

    #[test]
    fn part_2_test_solution() {
        let input = fs::read_to_string("src/day7/input.txt").unwrap();
        let res = day7(input);
        assert_eq!(res.part_2_total_splits, 16716444407407);
    }
}
