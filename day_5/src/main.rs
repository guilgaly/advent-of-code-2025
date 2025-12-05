use common::itertools::Itertools;
use common::sscanf::sscanf;
use common::time_execution;
use std::error::Error;

static INPUT_1: &str = include_str!("input1");
static INPUT_2: &str = include_str!("input2");

fn main() -> Result<(), Box<dyn Error>> {
    let fresh_ranges = INPUT_1
        .lines()
        .map(|line| sscanf!(line, "{u64}-{u64}").map(|(start, end)| Range { start, end }))
        .collect::<Result<Vec<Range>, _>>()?;
    let ingredients = INPUT_2
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<u64>, _>>()?;

    time_execution("Part 1", || part_1(&fresh_ranges, &ingredients));

    time_execution("Part 2", || part_2(&fresh_ranges));

    Ok(())
}

fn part_1(fresh_ranges: &[Range], ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| {
            fresh_ranges
                .iter()
                .find(|range| range.contains(**ingredient))
                .is_some()
        })
        .count()
}

fn part_2(fresh_ranges: &[Range]) -> u64 {
    let mut ranges = fresh_ranges.iter().cloned().collect_vec();
    loop {
        let mut merged_ranges: Vec<Range> = Vec::new();
        for r1 in ranges.iter() {
            match merged_ranges
                .iter_mut()
                .position(|r2| !(r1.end < r2.start || r1.start > r2.end))
            {
                Some(pos) => {
                    let r2 = merged_ranges[pos];
                    merged_ranges[pos] =
                        Range { start: r1.start.min(r2.start), end: r1.end.max(r2.end) }
                }
                None => {
                    merged_ranges.push(r1.clone());
                }
            }
        }
        if merged_ranges.len() == ranges.len() {
            ranges = merged_ranges;
            break;
        } else {
            ranges = merged_ranges;
        }
    }

    ranges.iter().map(|r| r.len()).sum()
}

impl Range {
    fn contains(&self, x: u64) -> bool {
        self.start <= x && x <= self.end
    }
    fn len(&self) -> u64 {
        self.end - self.start + 1
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Range {
    start: u64,
    end: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_RANGES: [Range; 4] = [
        Range { start: 3, end: 5 },
        Range { start: 10, end: 14 },
        Range { start: 16, end: 20 },
        Range { start: 12, end: 18 },
    ];
    static TEST_INGREDIENTS: [u64; 6] = [1, 5, 8, 11, 17, 32];

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_RANGES, &TEST_INGREDIENTS), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_RANGES), 14);
    }
}
