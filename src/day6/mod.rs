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
    let mut operations: Vec<bool> = vec![];
    let mut part_1_rows: Vec<Vec<u128>> = vec![];
    let mut part_2_columns: Vec<String> = vec![];
    for line in input.lines() {
        if line.contains('*') || line.contains('+') {
            operations = line
                .split(" ")
                .filter(|x| x.contains("*") || x.contains("+"))
                .map(|x| x.trim() == "+")
                .collect();
        } else {
            let row = line.split(" ").filter_map(|x| x.trim().parse::<u128>().ok()).collect();

            part_1_rows.push(row);

            let chars: Vec<char> = line.chars().collect();
            for i in 0..chars.len() {
                let c = chars.get(i).unwrap();
                if part_2_columns.get(i).is_none() {
                    part_2_columns.push(c.to_string());
                } else {
                    part_2_columns[i].push(*c);
                }
            }
        }
    }

    return calc(part_1_rows, part_2_columns, operations);
}

fn calc(part_1_rows: Vec<Vec<u128>>, part_2_columns: Vec<String>, operations: Vec<bool>) -> Day6Results {
    let mut output = Day6Results::new();
    let mut results: Vec<u128> = vec![];
    for i in 0..part_1_rows.len() {
        let row = part_1_rows.get(i).unwrap();
        for j in 0..row.len() {
            let is_addition = *operations.get(j).unwrap();
            let v = *row.get(j).unwrap();
            if results.get(j).is_none() {
                results.push(v);
            } else {
                results[j] = if is_addition { results[j] + v } else { results[j] * v };
            }
        }
    }
    output.part_1_grand_total = results.iter().sum::<u128>();

    let cols: Vec<&[String]> = part_2_columns.split(|p| p.trim().is_empty()).collect();
    for i in 0..operations.len() {
        let is_addition = *operations.get(i).unwrap();
        let values: &[String] = cols.get(i).unwrap();
        let val = values.iter().filter_map(|f| f.trim().parse::<u128>().ok());
        output.part_2_grand_total += if is_addition { val.sum::<u128>() } else { val.product::<u128>() };
    }
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

    #[test]
    fn part_1_test_solution() {
        let input = fs::read_to_string("src/day6/input.txt").unwrap();
        let res = day6(input);
        assert_eq!(res.part_1_grand_total, 5361735137219);
    }

    #[test]
    fn part_2_test_1() {
        let input = fs::read_to_string("src/day6/test.txt").unwrap();
        let res = day6(input);
        assert_eq!(res.part_2_grand_total, 3263827);
    }

    #[test]
    fn part_2_test_solution() {
        let input = fs::read_to_string("src/day6/input.txt").unwrap();
        let res = day6(input);
        assert_eq!(res.part_2_grand_total, 11744693538946);
    }
}
