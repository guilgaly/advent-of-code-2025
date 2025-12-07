use common::itertools::Itertools;
use common::maplit::{hashmap, hashset};
use common::time_execution;
use std::collections::HashMap;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let manifold = parse_input(INPUT)?;

    time_execution("Part 1", || part_1(&manifold));

    time_execution("Part 2", || part_2(&manifold));

    Ok(())
}

fn part_1(manifold: &Manifold) -> usize {
    let mut split_count = 0;
    let mut streams = hashset![manifold.start_position];
    for splitters_line in manifold.splitters.iter() {
        streams = streams
            .iter()
            .flat_map(|position| {
                if splitters_line.contains(position) {
                    split_count += 1;
                    vec![position - 1, position + 1]
                } else {
                    vec![*position]
                }
            })
            .collect()
    }
    split_count
}

fn part_2(manifold: &Manifold) -> usize {
    let mut streams = hashmap! { manifold.start_position => 1usize };
    for splitters_line in manifold.splitters.iter() {
        let mut new_streams = HashMap::new();
        for (position, count) in streams.into_iter() {
            if splitters_line.contains(&position) {
                new_streams
                    .entry(position - 1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                new_streams
                    .entry(position + 1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            } else {
                new_streams
                    .entry(position)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }
        streams = new_streams
    }
    streams.values().sum()
}

fn parse_input(input: &str) -> Result<Manifold, Box<dyn Error>> {
    let mut lines = input.lines();
    let start_x = lines
        .next()
        .ok_or("Empty input".to_string())?
        .chars()
        .position(|c| c == 'S')
        .ok_or("Missing start position".to_string())?;
    let splitters = lines
        .map(|line| line.chars().positions(|c| c == '^').collect_vec())
        .collect_vec();
    Ok(Manifold { start_position: start_x, splitters })
}

struct Manifold {
    start_position: usize,
    splitters: Vec<Vec<usize>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_INPUT: Manifold = parse_input(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        )
        .unwrap();
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_INPUT), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_INPUT), 40);
    }
}
