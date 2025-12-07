use common::itertools::Itertools;
use common::time_execution_res;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    time_execution_res("Part 1", || part_1(INPUT))?;

    time_execution_res("Part 2", || part_2(INPUT))?;

    Ok(())
}

fn part_1(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut lines = input.lines().collect_vec();
    let ops = lines
        .pop()
        .ok_or("Empty input")?
        .split_whitespace()
        .map(|x| {
            if x == "*" {
                Ok(Operator::Mult)
            } else if x == "+" {
                Ok(Operator::Add)
            } else {
                Err(format!("{:?} is not an operator", x))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut operations = ops
        .iter()
        .map(|&op| Operation { numbers: Vec::new(), op })
        .collect_vec();
    for line in lines.iter() {
        for (idx, entry) in line.split_whitespace().enumerate() {
            operations
                .get_mut(idx)
                .ok_or(format!("Missing operation at idx {}", idx))?
                .numbers
                .push(entry.parse()?);
        }
    }

    Ok(compute_operations(&operations))
}

fn part_2(input: &str) -> Result<u64, Box<dyn Error>> {
    // Parse each line separately (character by character)
    let mut lines = input.lines().collect_vec();
    let ops_line = lines
        .pop()
        .ok_or("Empty input")?
        .chars()
        .map(|c| match c {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Mult),
            _ => None,
        })
        .collect_vec();
    let number_lines = lines
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).map(|d| d as u64))
                .collect_vec()
        })
        .collect_vec();

    // Move form right to left
    let mut operations = Vec::new();
    let mut x = ops_line.len() - 1;
    let mut numbers = Vec::new();
    loop {
        let number = number_lines
            .iter()
            .fold(0, |acc, line| line[x].map(|d| 10 * acc + d).unwrap_or(acc));
        numbers.push(number);
        match ops_line[x] {
            None => {
                x -= 1;
            }
            Some(op) => {
                operations.push(Operation { numbers, op });
                numbers = Vec::new();
                if x == 0 {
                    break;
                } else {
                    x -= 2;
                }
            }
        }
    }

    Ok(compute_operations(&operations))
}

fn compute_operations(operations: &[Operation]) -> u64 {
    operations
        .iter()
        .map(|Operation { op, numbers }| match op {
            Operator::Mult => numbers.iter().fold(1, |acc, x| acc * x),
            Operator::Add => numbers.iter().fold(0, |acc, x| acc + x),
        })
        .sum()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Operation {
    numbers: Vec<u64>,
    op: Operator,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Operator {
    Mult,
    Add,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  "
    );

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_INPUT).unwrap(), 4277556);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_INPUT).unwrap(), 3263827);
    }
}
