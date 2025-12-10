use common::itertools::Itertools;
use common::sscanf::sscanf;
use common::time_execution;
use std::error::Error;
use std::ops;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let tiles = parse_input(INPUT)?;

    time_execution("Part 1", || part_1(&tiles));

    time_execution("Part 2", || part_2(&tiles));

    Ok(())
}

fn part_1(tiles: &[Point]) -> i64 {
    tiles
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(p1, p2)| Rectangle::new(p1, p2).area())
        .max()
        .unwrap_or(0)
}

fn part_2(tiles: &[Point]) -> i64 {
    let mut circumference = tiles
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(p1, p2)| Segment::new(p1, p2))
        .collect_vec();
    circumference.push(Segment::new(&tiles[0], &tiles[tiles.len() - 1]));

    tiles
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(p1, p2)| Rectangle::new(p1, p2))
        .sorted_by_key(|r| r.area())
        .rev()
        .find(|r| {
            circumference
                .iter()
                .find(|seg| r.is_intersected_by(seg))
                .is_none()
        })
        .map(|r| r.area())
        .unwrap_or(0)
}

fn parse_input(input: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let (x, y) = sscanf!(line, "{i64},{i64}")?;
            Ok(Point { x, y })
        })
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn new(p1: &Point, p2: &Point) -> Rectangle {
        Rectangle {
            top_left: Point { x: p1.x.min(p2.x), y: p1.y.max(p2.y) },
            bottom_right: Point { x: p1.x.max(p2.x), y: p1.y.min(p2.y) },
        }
    }
    fn area(&self) -> i64 {
        (self.bottom_right.x - self.top_left.x + 1) * (self.top_left.y - self.bottom_right.y + 1)
    }
    /** True if the segment crosses inside the rectangle, but not if it only touches the borders. */
    fn is_intersected_by(&self, seg: &Segment) -> bool {
        match seg {
            Segment::Vert { x, y_min, y_max } => {
                self.top_left.x < *x
                    && *x < self.bottom_right.x
                    && *y_min < self.top_left.y
                    && *y_max > self.bottom_right.y
            }
            Segment::Horiz { x_min, x_max, y } => {
                self.bottom_right.y < *y
                    && *y < self.top_left.y
                    && *x_min < self.bottom_right.x
                    && *x_max > self.top_left.x
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Segment {
    Vert { x: i64, y_min: i64, y_max: i64 },
    Horiz { x_min: i64, x_max: i64, y: i64 },
}

impl Segment {
    fn new(p1: &Point, p2: &Point) -> Segment {
        if p1.x == p2.x {
            Segment::Vert { x: p1.x, y_min: p1.y.min(p2.y), y_max: p1.y.max(p2.y) }
        } else if p1.y == p2.y {
            Segment::Horiz { x_min: p1.x.min(p2.x), x_max: p1.x.max(p2.x), y: p1.y }
        } else {
            panic!();
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [Point; 8] = [
        Point { x: 7, y: 1 },
        Point { x: 11, y: 1 },
        Point { x: 11, y: 7 },
        Point { x: 9, y: 7 },
        Point { x: 9, y: 5 },
        Point { x: 2, y: 5 },
        Point { x: 2, y: 3 },
        Point { x: 7, y: 3 },
    ];

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&INPUT), 50);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&INPUT), 24);
    }
}
