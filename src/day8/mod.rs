use std::fmt;

pub struct Day8Results {
    pub part_1_total_splits: i128,
    pub part_2_total_splits: u128,
}

#[derive(Default)]
struct Connections {
    pub positions: Vec<Vector3i>,
}

#[derive(Clone, Copy, PartialEq, Default)]
struct Vector3i {
    x: i128,
    y: i128,
    z: i128,
}

impl Vector3i {
    pub fn distance_to(&self, other: Vector3i) -> u128 {
        (self.x - other.x).pow(2) as u128 + (self.y - other.y).pow(2) as u128 + (self.z - other.z).pow(2) as u128
    }
}

impl fmt::Display for Vector3i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
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
    let mut coords: Vec<Vector3i> = vec![];
    for line in input.lines() {
        let l: Vec<i128> = line.split(",").filter_map(|c| c.parse::<i128>().ok()).collect();
        let pos: Vector3i = Vector3i { x: l[0], y: l[1], z: l[2] };
        coords.push(pos);
    }
    return calc(coords);
}

fn calc(coordinates: Vec<Vector3i>) -> Day8Results {
    let mut connections: Vec<Connections> = vec![];

    'outer: for i in 0..coordinates.len() {
        let mut coord = coordinates.get(i).unwrap();
        let mut dist = u128::MAX;
        let mut closest: Vector3i = Vector3i::default();
        for j in 0..coordinates.len() {
            let compare = *coordinates.get_mut(j).unwrap();
            if *coord == compare {
                continue;
            }
            let calc = coord.distance_to(compare);
            if dist > calc {
                dist = calc;
                closest = compare;
            }
        }
        for mut connection in connections {
            if connection.positions.contains(&closest) {
                connection.positions.push(*coord);
                continue 'outer;
            }
        }

        let mut new_conn = Connections { positions: vec![] };
        new_conn.positions.push(closest);
        new_conn.positions.push(*coord);
        connections.push(new_conn);
    }

    let mut output = Day8Results::new();
    for c in connections {
        output.part_1_total_splits += c.positions.len() as i128;
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
