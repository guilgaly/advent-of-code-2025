use common::itertools::Itertools;
use common::sscanf::sscanf;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    // e.g. read lines
    let lines = INPUT.lines().collect_vec();
    // e.g. parse lines with sscanf
    let _parsed_lines = INPUT
        .lines()
        .map(|line| sscanf!(line, "{usize},{str}"))
        .collect::<Result<Vec<_>, _>>()?;

    let res1 = part_1(&lines);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&lines);
    println!("Part 2 result: {}", res2);

    Ok(())
}

fn part_1(lines: &[&str]) -> usize {
    lines.len()
}

fn part_2(lines: &[&str]) -> usize {
    lines.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = ["A", "B"];
        assert_eq!(part_1(&test_input), 2);
    }

    #[test]
    fn test_part_2() {
        let test_input = ["A", "B"];
        assert_eq!(part_2(&test_input), 2);
    }
}
