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
#[derive(Clone, Copy)]
struct Vector3i {
    x: i128,
    y: i128,
    z: i128,
}

impl Vector3i {
    fn mgn(self) -> i128 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64) as i128
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
    let output: Day8Results = Day8Results::new();
    let mut pairs: Vec<(JunctionBox, JunctionBox)> = vec![];
    for j in &junction_boxes {
        let mut dist = MAX;
        let mut index = 0;
        for i in 0..junction_boxes.len() {
            let b = junction_boxes.get(i).unwrap();
            if j.equals(*b) {
                continue;
            }
            let new_dist = (j.position.mgn() - b.position.mgn()).abs();
            if dist > new_dist {
                dist = new_dist;
                index = i;
            }
        }
        let b: &JunctionBox = junction_boxes.get(index).unwrap();
        pairs.push((*j, *b));
    }
    for pair in pairs {
        println!("{} - {}", pair.0, pair.1);
    }
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
