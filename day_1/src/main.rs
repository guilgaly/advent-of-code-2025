use common::time_execution;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let rotations = parse_rotations(&INPUT)?;

    time_execution("Part 1", || part_1(&rotations));

    time_execution("Part 2", || part_2(&rotations));

    Ok(())
}

fn part_1(rotations: &[i64]) -> u64 {
    let mut password = 0;
    let mut position = 50;

    for rotation in rotations {
        let effective_rotation = rotation % 100;
        let raw_position = position + effective_rotation;
        position = if raw_position < 0 {
            100 + raw_position
        } else if raw_position > 99 {
            raw_position - 100
        } else {
            raw_position
        };
        if position == 0 {
            password += 1;
        }
    }

    password
}

fn part_2(rotations: &[i64]) -> u64 {
    let mut password = 0;
    let mut position = 50;

    for rotation in rotations {
        // Count all full rotations first, since they simply bring us back to the same position
        password += (rotation / 100).unsigned_abs();

        // Apply remaining clicks
        let starting_position = position;
        let effective_rotation = rotation % 100;
        if effective_rotation != 0 {
            let raw_position = starting_position + effective_rotation;
            position = if raw_position < 0 {
                100 + raw_position
            } else if raw_position > 99 {
                raw_position - 100
            } else {
                raw_position
            };

            // Count if the remaining clicks included 0. There is one subtle case: do not count when
            // going left _from zero_ (that zero was already counted on the previous rotation)
            if (starting_position != 0 && raw_position <= 0) || raw_position >= 100 {
                password += 1;
            }
        }
    }

    password
}

fn parse_rotations(input: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let (first, last) = line.split_at(1);
            let count: i64 = last.parse()?;
            if first == "R" {
                Ok(count)
            } else if first == "L" {
                Ok(-count)
            } else {
                Err("Invalid rotation".into())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = [-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        assert_eq!(part_1(&test_input), 3);
    }

    #[test]
    fn test_part_2() {
        let test_input = [-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        assert_eq!(part_2(&test_input), 6);
    }
}
