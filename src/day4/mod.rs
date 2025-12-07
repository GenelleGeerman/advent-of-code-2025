use std::collections::HashMap;

pub struct Day4Results {
    pub part_1_rolls: u128,
    pub part_2_rolls: u128,
}

impl Day4Results {
    fn new() -> Self {
        Self {
            part_1_rolls: 0,
            part_2_rolls: 0,
        }
    }
}

pub fn day4(input: String) -> Day4Results {
    let mut input_matrix: HashMap<(i32, i32), char> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        for char in line.chars() {
            input_matrix.insert((x, y), char);
            x += 1;
        }
        x = 0;
        y += 1;
    }
    return calc(input_matrix);
}

fn calc(input: HashMap<(i32, i32), char>) -> Day4Results {
    let mut output: Day4Results = Day4Results::new();

    for key in input.keys() {
        let value = *input.get(key).unwrap();
        if !is_roll(value) {
            continue;
        }
        let positions = possible_positions(*key);
        let mut nearby_rolls = 0;
        for pos in positions {
            let pos_key = input.get(&pos);
            if pos_key.is_none() {
                continue;
            }
            if is_roll(*pos_key.unwrap()) {
                nearby_rolls += 1;
                if nearby_rolls >= 4 {
                    break;
                }
            }
        }
        if nearby_rolls < 4 {
            output.part_1_rolls += 1;
        }
    }
    return output;
}

fn is_roll(c: char) -> bool {
    c == '@'
}

fn possible_positions(key: (i32, i32)) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = vec![];
    output.push((key.0 - 1, key.1 - 1));
    output.push((key.0 - 1, key.1));
    output.push((key.0 - 1, key.1 + 1));
    output.push((key.0, key.1 - 1));
    output.push((key.0, key.1 + 1));
    output.push((key.0 + 1, key.1 - 1));
    output.push((key.0 + 1, key.1));
    output.push((key.0 + 1, key.1 + 1));
    return output;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_test_1() {
        let input =
            "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let res = day4(String::from(input));
        assert_eq!(res.part_1_rolls, 13);
    }

    #[test]
    fn part_1_test_solution() {
        let input = fs::read_to_string("src/day3/input.txt").unwrap();

        let res = day4(String::from(input));
        assert_eq!(res.part_1_rolls, 1445);
    }

    fn part_1_test_1() {
        let input =
            "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let res = day4(String::from(input));
        assert_eq!(res.part_2_rolls, 43);
    }
}
