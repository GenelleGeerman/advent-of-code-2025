pub struct Day6Results {
    pub part_1_grand_total: u128,
    pub part_2_grand_total: u128,
}

impl Day6Results {
    fn new() -> Self {
        Self {
            part_1_grand_total: 0,
            part_2_grand_total: 0,
        }
    }
}

pub fn day6(input: String) -> Day6Results {
    let mut rows: Vec<Vec<u128>> = vec![];
    let mut operations: Vec<bool> = vec![];
    for line in input.lines() {
        if line.contains('*') || line.contains('+') {
            operations = line.split(" ").filter(|x| !x.contains(" ")).map(|x| x == "+").collect();
        } else {
            rows.push(line.split(" ").filter_map(|x| x.parse().ok()).collect());
        }
    }
    let mut results: Vec<u128> = vec![];
    for row in rows {
        for i in 0..row.len() {
            let column = *row.get(i).unwrap();

            if *operations.get(i).unwrap() {
                let prev = *results.get(i).unwrap_or(&0);

                if results.get(i).is_none() {
                    results.insert(i, prev);
                }
                results[i] = prev + column;
            } else {
                let prev = *results.get(i).unwrap_or(&1);

                if results.get(i).is_none() {
                    results.insert(i, prev);
                }
                results[i] = prev * column;
            }
        }
    }
    let mut output = Day6Results::new();
    output.part_1_grand_total = results.iter().sum();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_1() {
        let input = fs::read_to_string("src/day6/test.txt").unwrap();
        let res = day6(input);
        assert_eq!(res.part_1_grand_total, 4277556);
    }
}
