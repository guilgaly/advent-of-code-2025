use common::itertools::Itertools;
use common::maplit::hashset;
use common::sscanf::sscanf;
use common::time_execution;
use std::collections::HashSet;
use std::error::Error;
use z3::ast::Int;
use z3::{Optimize, SatResult};

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    // e.g. read lines
    let machines = parse_input(INPUT)?;

    time_execution("Part 1", || part_1(&machines));

    time_execution("Part 2", || part_2(&machines));

    Ok(())
}

fn part_1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| {
            let mut lights_states = hashset! { vec![false; machine.lights.len()] };
            let mut btn_presses = 0;
            while lights_states
                .iter()
                .find(|lights| **lights == machine.lights)
                .is_none()
            {
                let mut new_states = HashSet::new();
                for prev_state in lights_states.iter() {
                    for button in machine.buttons.iter() {
                        let mut new_state = prev_state.clone();
                        for btn_i in button.iter() {
                            new_state[*btn_i] = !new_state[*btn_i];
                        }
                        new_states.insert(new_state);
                    }
                }
                lights_states = new_states;
                btn_presses += 1;
            }
            btn_presses
        })
        .sum()
}

fn part_2(machines: &[Machine]) -> u64 {
    machines
        .iter()
        .map(|machine| {
            let optimize = Optimize::new();
            let btn_counts = machine
                .buttons
                .iter()
                .enumerate()
                .map(|(i, _)| Int::fresh_const(&format!("btn_count_{}", i)))
                .collect_vec();
            for btn_count in btn_counts.iter() {
                optimize.assert(&btn_count.ge(0));
            }
            for i in 0..machine.joltages.len() {
                let sum = machine
                    .buttons
                    .iter()
                    .enumerate()
                    .flat_map(|(btn_idx, btn)| {
                        if btn.contains(&i) {
                            Some(&btn_counts[btn_idx])
                        } else {
                            None
                        }
                    })
                    .fold(Int::from_u64(0), |acc, x| acc + x);
                optimize.assert(&sum.eq(machine.joltages[i]))
            }
            let total_sum = btn_counts.iter().fold(Int::from_u64(0), |acc, x| acc + x);
            optimize.minimize(&total_sum);
            // TODO clean up error handling...
            if optimize.check(&[]) != SatResult::Sat {
                panic!()
            }
            optimize
                .get_model()
                .unwrap()
                .eval(&total_sum, true)
                .unwrap()
                .as_u64()
                .unwrap()
        })
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Machine>, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let (s1, s2, s3) = sscanf!(line, "[{str}] {str} {{{str}}}")?;
            let lights = s1.chars().map(|c| c == '#').collect_vec();
            let buttons = s2
                .split_whitespace()
                .map(|s| {
                    let list = sscanf!(s, "({str})")?;
                    let values = list
                        .split(",")
                        .map(|v| v.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok::<_, Box<dyn Error>>(values)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let joltages = s3
                .split(",")
                .map(|s| s.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Machine { lights, buttons, joltages })
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_INPUT: Vec<Machine> = parse_input(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
        )
        .unwrap();
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_INPUT), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_INPUT), 33);
    }
}
