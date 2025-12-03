use common::time_execution;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let battery_banks = parse_input(INPUT)?;

    time_execution("Part 1", || part_1(&battery_banks));

    time_execution("Part 2", || part_2(&battery_banks));

    Ok(())
}

fn part_1(battery_banks: &Vec<Vec<u64>>) -> u64 {
    max_joltage(battery_banks, 2)
}

fn part_2(battery_banks: &Vec<Vec<u64>>) -> u64 {
    max_joltage(battery_banks, 12)
}

fn max_joltage(battery_banks: &Vec<Vec<u64>>, batteries_per_bank: u32) -> u64 {
    fn recurs(bank: &[u64], remaining: u32) -> u64 {
        if remaining == 1 {
            *bank.iter().max().unwrap()
        } else {
            // We always get the max total value by first searching for the max value for the most
            // significant digit (while leaving as much search space as possible for the following
            // digits, and in any case at least enough for the number of expected digits).
            let digit_value = bank[0..(bank.len() - (remaining as usize) + 1)]
                .iter()
                .max()
                .unwrap();
            // max() returns the last occurrence of the max value, so we have to do a separate
            // search for the position of its first occurrence.
            let digit_pos = bank.iter().position(|d| d == digit_value).unwrap();
            10u64.pow(remaining - 1) * digit_value + recurs(&bank[(digit_pos + 1)..], remaining - 1)
        }
    }
    battery_banks
        .iter()
        .map(|bank| recurs(bank, batteries_per_bank))
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            if line.len() < 12 {
                return Err(format!("Not enough batteries in bank {}", line).into());
            }
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u64)
                        .ok_or(format!("Invalid character '{}'", c).into())
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_INPUT: Vec<Vec<u64>> = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_INPUT), 357);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_INPUT), 3121910778619);
    }
}
