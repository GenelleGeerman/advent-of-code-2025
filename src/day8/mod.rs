use std::{collections::HashSet, fmt::Display, hash::Hash, ops::Index};

#[derive(Default)]
pub struct Day8Results {
    pub part_1_total_connections: i64,
    pub part_2_total_connections: u128,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub fn day8(input: String, limiter: usize) -> Day8Results {
    let mut coords: Vec<Coord> = vec![];
    for line in input.lines() {
        let l: Vec<i64> = line.split(",").filter_map(|c| c.parse().ok()).collect();
        let pos = Coord { x: l[0], y: l[1], z: l[2] };
        coords.push(pos);
    }
    return calc(coords, limiter);
}

fn calc(coordinates: Vec<Coord>, limiter: usize) -> Day8Results {
    let mut output = Day8Results::default();
    let distances: Vec<(i64, Coord, Coord)> = get_distances(&coordinates);
    let total = coordinates.len();
    let (mut connections, part2_edge) = get_connections(distances, limiter, total);

    connections.sort_by_key(|conn| conn.len());
    connections.reverse();
    output.part_1_total_connections = connections.iter().take(3).map(|f| f.len()).product::<usize>() as i64;
    if limiter == 0 {
        output.part_2_total_connections = {
            let (a, b) = part2_edge.unwrap();
            (a.x * b.x) as u128
        };
    }

    return output;
}

fn get_distances(coordinates: &Vec<Coord>) -> Vec<(i64, Coord, Coord)> {
    let mut distances: Vec<(i64, Coord, Coord)> = Vec::new();
    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let a = coordinates[i];
            let b = coordinates[j];
            let dist = distance(a, b);
            distances.push((dist as i64, a, b));
        }
    }
    distances.sort_by_key(|(distance, _, _)| *distance);
    return distances;
}

fn distance(p: Coord, q: Coord) -> i64 {
    let dx = p.x - q.x;
    let dy = p.y - q.y;
    let dz = p.z - q.z;
    dx * dx + dy * dy + dz * dz
}

fn get_connections(distances: Vec<(i64, Coord, Coord)>, mut limiter: usize, total: usize) -> (Vec<HashSet<Coord>>, Option<(Coord, Coord)>) {
    let mut connections: Vec<HashSet<Coord>> = vec![];
    let mut part2_edge: Option<(Coord, Coord)> = None;
    if limiter == 0 {
        limiter = distances.len();
    }
    for (_, a, b) in distances.iter().take(limiter) {
        let mut index_a = None;
        let mut index_b = None;

        for (i, conn) in connections.iter().enumerate() {
            if conn.contains(&a) {
                index_a = Some(i);
            }
            if conn.contains(&b) {
                index_b = Some(i);
            }
        }

        match (index_a, index_b) {
            (Some(i), Some(j)) if i == j => {}
            (Some(i), Some(j)) => {
                let target = i.min(j);
                let other = connections.remove(j.max(i));
                connections[target].extend(other);
                if connections[target].len() == total && part2_edge.is_none() {
                    part2_edge = Some((*a, *b));
                }
            }
            (Some(i), None) => {
                connections[i].insert(*b);
                if connections[i].len() == total && part2_edge.is_none() {
                    part2_edge = Some((*a, *b));
                }
            }
            (None, Some(i)) => {
                connections[i].insert(*a);
                if connections[i].len() == total && part2_edge.is_none() {
                    part2_edge = Some((*a, *b));
                }
            }
            (None, None) => {
                let mut s = HashSet::new();
                s.insert(*a);
                s.insert(*b);
                connections.push(s);
            }
        }
    }
    return (connections, part2_edge);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test_1() {
        let input = fs::read_to_string("src/day8/test.txt").unwrap();
        let res = day8(input, 10);
        assert_eq!(res.part_1_total_connections, 40);
    }
    #[test]
    fn part_1_test_solution() {
        let input = fs::read_to_string("src/day8/input.txt").unwrap();
        let res = day8(input, 1000);
        assert_eq!(res.part_1_total_connections, 90036);
    }
    #[test]
    fn part_2_test_test_1() {
        let input = fs::read_to_string("src/day8/test.txt").unwrap();
        let res = day8(input, 0);
        assert_eq!(res.part_2_total_connections, 25272);
    }
    #[test]
    fn part_2_test_test_solution() {
        let input = fs::read_to_string("src/day8/input.txt").unwrap();
        let res = day8(input, 0);
        assert_eq!(res.part_2_total_connections, 6083499488);
    }
}
