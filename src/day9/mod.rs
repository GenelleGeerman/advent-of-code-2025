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

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Box {
    topLeft: Coord,
    topRight: Coord,
    bottomLeft: Coord,
    bottomRight: Coord,
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
    let mut largest2 = 0;
    let mut boxes: Vec<Edge> = vec![];
    let iter_coordinate = coordinates.clone();
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

            let ab = Coord { x: a.x, y: b.y };
            let ba = Coord { x: b.x, y: a.y };
            if !coordinates.contains(&ab) {
                if !is_within(ab, &iter_coordinate) {
                    continue;
                }
            }
            if !coordinates.contains(&ba) {
                if !is_within(ba, &iter_coordinate) {
                    continue;
                }
            }
            largest2 = largest;
        }
    }
    output.part_1_largest_area = largest;
    output.part_2_largest_area = largest2;
    return output;
}

fn is_within(cell: Coord, coordinates: &Vec<Coord>) -> bool {
    (coordinates.iter().filter(|coord| coord.y == cell.y).any(|coord| coord.x > cell.x)
        && coordinates.iter().filter(|coord| coord.y == cell.y).any(|coord| coord.x < cell.x))
        || (coordinates.iter().filter(|coord| coord.x == cell.x).any(|coord| coord.y > cell.y)
            && coordinates.iter().filter(|coord| coord.x == cell.x).any(|coord| coord.y < cell.y))
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
