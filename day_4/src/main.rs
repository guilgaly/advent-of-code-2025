use common::itertools::Itertools;
use common::time_execution;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let warehouse = parse_input(INPUT)?;

    time_execution("Part 1", || part_1(&warehouse));

    time_execution("Part 2", || part_2(&warehouse));

    Ok(())
}

fn part_1(warehouse: &Warehouse) -> usize {
    accessible_rolls(warehouse).len()
}

fn part_2(warehouse: &Warehouse) -> usize {
    let mut warehouse = warehouse.clone();
    let mut removed = 0;

    while let accessible = accessible_rolls(&warehouse)
        && !accessible.is_empty()
    {
        for p in accessible.iter() {
            warehouse.set(&p, false);
        }
        removed += accessible.len();
    }

    removed
}

fn accessible_rolls(warehouse: &Warehouse) -> Vec<Point> {
    warehouse
        .iter_values()
        .filter_map(|(p, is_occupied)| {
            let is_accessible =
                is_occupied && warehouse.neighbors(&p).filter(|(_, o)| *o).count() < 4;
            if is_accessible { Some(p) } else { None }
        })
        .collect_vec()
}

fn parse_input(input: &str) -> Result<Warehouse, Box<dyn Error>> {
    let paper_rolls = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect_vec())
        .collect_vec();
    let y_max = paper_rolls.len() - 1;
    let x_max = paper_rolls[0].len() - 1;
    Ok(Warehouse { paper_rolls, y_max, x_max })
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Warehouse {
    fn get(&self, p: &Point) -> bool {
        self.paper_rolls[p.y][p.x]
    }
    fn set(&mut self, p: &Point, value: bool) {
        self.paper_rolls[p.y][p.x] = value;
    }
    fn iter(&self) -> impl Iterator<Item = Point> {
        (0..=self.x_max).flat_map(|x| (0..=self.y_max).map(move |y| Point { x, y }))
    }
    fn iter_values(&self) -> impl Iterator<Item = (Point, bool)> {
        self.iter().map(|p| (p, self.get(&p)))
    }
    fn neighbors(&self, &Point { x, y }: &Point) -> impl Iterator<Item = (Point, bool)> {
        let x0 = if x == 0 { x } else { x - 1 };
        let x1 = if x == self.x_max { x } else { x + 1 };
        let y0 = if y == 0 { y } else { y - 1 };
        let y1 = if y == self.y_max { y } else { y + 1 };
        (x0..=x1).into_iter().flat_map(move |n_x| {
            (y0..=y1).into_iter().filter_map(move |n_y| {
                if n_x != x || n_y != y {
                    let n = Point { x: n_x, y: n_y };
                    Some((n, self.get(&n)))
                } else {
                    None
                }
            })
        })
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
struct Warehouse {
    x_max: usize,
    y_max: usize,
    paper_rolls: Vec<Vec<bool>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_WAREHOUSE: Warehouse = parse_input(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
        )
        .unwrap();
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_WAREHOUSE), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_WAREHOUSE), 43);
    }
}
