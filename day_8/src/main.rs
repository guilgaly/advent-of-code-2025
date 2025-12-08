use common::itertools::Itertools;
use common::sscanf::sscanf;
use common::time_execution;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    // e.g. read lines
    let junction_boxes = parse_input(INPUT)?;

    time_execution("Part 1", || part_1(&junction_boxes, 1000));

    time_execution("Part 2", || part_2(&junction_boxes));

    Ok(())
}

fn part_1(junction_boxes: &[JunctionBox], max_connections: usize) -> usize {
    let closest_pairs = closest_pairs(junction_boxes).take(max_connections);

    let mut circuits = junction_boxes.iter().map(|j| vec![j]).collect_vec();
    for (p1, p2) in closest_pairs {
        let pos1 = circuits.iter().position(|c| c.contains(&&p1)).unwrap();
        let pos2 = circuits.iter().position(|c| c.contains(&&p2)).unwrap();
        if pos1 != pos2 {
            let c1_pos = pos1.min(pos2);
            let c2_pos = pos1.max(pos2);
            let c2 = circuits.remove(c2_pos);
            circuits[c1_pos].extend(c2.iter());
        }
    }

    circuits
        .iter()
        .map(|circuit| circuit.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn part_2(junction_boxes: &[JunctionBox]) -> i64 {
    let mut circuits = junction_boxes.iter().map(|j| vec![j]).collect_vec();
    for (p1, p2) in closest_pairs(junction_boxes) {
        let pos1 = circuits.iter().position(|c| c.contains(&&p1)).unwrap();
        let pos2 = circuits.iter().position(|c| c.contains(&&p2)).unwrap();
        if pos1 != pos2 {
            let c1_pos = pos1.min(pos2);
            let c2_pos = pos1.max(pos2);
            let c2 = circuits.remove(c2_pos);
            circuits[c1_pos].extend(c2.iter());
        }
        if circuits.len() == 1 {
            return p1.x * p2.x;
        }
    }
    0
}

fn closest_pairs(
    junction_boxes: &[JunctionBox],
) -> impl Iterator<Item = (JunctionBox, JunctionBox)> {
    junction_boxes
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(p1, p2)| (p1.distance_to_squared(p2), p1, p2))
        .sorted_by_key(|(dist, _, _)| *dist)
        .map(|(_, id1, id2)| (*id1, *id2))
}

fn parse_input(input: &str) -> Result<Vec<JunctionBox>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = sscanf!(line, "{i64},{i64},{i64}")?;
            Ok(JunctionBox { x, y, z })
        })
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn distance_to_squared(&self, other: &JunctionBox) -> i64 {
        // Actual distance is the square root of this, but for comparisons we don't care
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_INPUT: Vec<JunctionBox> = parse_input(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
        )
        .unwrap();
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_INPUT, 10), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_INPUT), 25272);
    }
}
