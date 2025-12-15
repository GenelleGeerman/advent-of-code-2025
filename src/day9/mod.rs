use std::{fmt::Display, hash::Hash};

#[derive(Default)]
pub struct Day9Results {
    pub part_1_largest_area: i64,
    pub part_2_largest_area: i64,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    left: Coord,
    right: Coord,
}

impl Edge {
    fn contains(&self, coord: Coord) -> bool {
        self.left.x < coord.x && self.right.x > coord.x && self.left.y < coord.y && self.right.y > coord.y
    }
}

pub fn day9(input: String) -> Day9Results {
    let mut coords: Vec<Coord> = vec![];
    for line in input.lines() {
        let l: Vec<i64> = line.split(",").filter_map(|c| c.parse().ok()).collect();
        let pos = Coord { x: l[0], y: l[1] };
        coords.push(pos);
    }
    return calc(coords);
}

fn calc(coordinates: Vec<Coord>) -> Day9Results {
    let mut output = Day9Results::default();

    let mut largest = 0;
    let mut edges: Vec<Edge> = vec![];
    for i in 0..coordinates.len() {
        let a = coordinates.get(i).unwrap();
        for j in i + 1..coordinates.len() {
            let b = coordinates.get(j).unwrap();
            let area = ((b.x - a.x).abs() + 1) * ((b.y - a.y).abs() + 1);
            if area > largest {
                largest = area;
            }

            if a.x == b.x || a.y == b.y {
                edges.push(Edge { left: *a, right: *b })
            }
        }
    }
    //  if largest2 <= area && (coordinates.contains(&Coord { x: a.x, y: b.y }) || coordinates.contains(&Coord { x: b.x, y: a.y })) {
    //             println!("{area}");
    //             largest2 = area;
    //         }
    output.part_1_largest_area = largest;
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_1() {
        let input = fs::read_to_string("src/day9/test.txt").unwrap();
        let res = day9(input);
        assert_eq!(res.part_1_largest_area, 50);
    }
    #[test]
    fn part_1_test_solution() {
        let input = fs::read_to_string("src/day9/input.txt").unwrap();
        let res = day9(input);
        assert_eq!(res.part_1_largest_area, 4769758290);
    }
    #[test]
    fn part_2_test_1() {
        let input = fs::read_to_string("src/day9/test.txt").unwrap();
        let res = day9(input);
        assert_eq!(res.part_2_largest_area, 24);
    }
}
