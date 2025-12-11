use std::{fmt, i128::MAX};

pub struct Day8Results {
    pub part_1_total_splits: i128,
    pub part_2_total_splits: u128,
}

#[derive(Clone, Copy)]
struct JunctionBox {
    pub position: Vector3i,
    pub is_connected: bool,
}

impl PartialEq for JunctionBox {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Vector3i {
    x: i128,
    y: i128,
    z: i128,
}

impl Vector3i {
    pub fn distance_to(&self, other: Vector3i) -> i128 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }
}

impl fmt::Display for JunctionBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.position.x, self.position.y, self.position.z)
    }
}

impl JunctionBox {
    fn equals(&self, b: JunctionBox) -> bool {
        self.position.x == b.position.x && self.position.y == b.position.y && self.position.z == b.position.z
    }
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
    let mut junction_boxes: Vec<JunctionBox> = vec![];
    for line in input.lines() {
        let l: Vec<i128> = line.split(",").filter_map(|c| c.parse::<i128>().ok()).collect();
        let pos: Vector3i = Vector3i { x: l[0], y: l[1], z: l[2] };

        junction_boxes.push(JunctionBox {
            position: pos,
            is_connected: false,
        });
    }
    return calc(junction_boxes);
}

fn calc(junction_boxes: Vec<JunctionBox>) -> Day8Results {
    Day8Results {
        part_1_total_splits: 0,
        part_2_total_splits: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_1() {
        let input = fs::read_to_string("src/day8/test.txt").unwrap();
        let res = day8(input);
        assert_eq!(res.part_1_total_splits, 40);
    }

    // #[test]
    // fn part_1_test_solution() {
    //     let input = fs::read_to_string("src/day8/input.txt").unwrap();
    //     let res = day8(input);
    //     assert_eq!(res.part_1_total_splits, 1585);
    // }

    // #[test]
    // fn part_2_test_1() {
    //     let input = fs::read_to_string("src/day8/test.txt").unwrap();
    //     let res = day8(input);
    //     assert_eq!(res.part_2_total_splits, 40);
    // }

    // #[test]
    // fn part_2_test_solution() {
    //     let input = fs::read_to_string("src/day8/input.txt").unwrap();
    //     let res = day8(input);
    //     assert_eq!(res.part_2_total_splits, 16716444407407);
    // }
}
