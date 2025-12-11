use common::itertools::Itertools;
use common::sscanf::sscanf;
use common::time_execution;
use std::collections::HashMap;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let devices = parse_input(INPUT)?;

    time_execution("Part 1", || part_1(&devices));

    time_execution("Part 2", || part_2(&devices));

    Ok(())
}

fn part_1(devices: &Devices) -> usize {
    // A simple recursive implementation of DFS works fine here
    fn recurs_dfs(devices: &Devices, curr: &str) -> usize {
        if curr == "out" {
            1
        } else {
            devices
                .get(curr)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|out| recurs_dfs(devices, out))
                .sum()
        }
    }

    recurs_dfs(&devices, "you")
}

fn part_2(devices: &Devices) -> usize {
    // Basically the same recursive DFS, but with an added cache
    fn recurs_dfs<'a>(
        devices: &'a Devices,
        cache: &mut HashMap<(&'a str, bool, bool), usize>,
        curr: &'a str,
        with_dac: bool,
        with_fft: bool,
    ) -> usize {
        if curr == "out" {
            if with_dac && with_fft { 1 } else { 0 }
        } else {
            match cache.get(&(curr, with_dac, with_fft)) {
                Some(res) => *res,
                None => {
                    let with_dac = with_dac || curr == "dac";
                    let with_fft = with_fft || curr == "fft";
                    let res = devices
                        .get(curr)
                        .map(|outputs| {
                            outputs
                                .iter()
                                .map(|out| recurs_dfs(devices, cache, out, with_dac, with_fft))
                                .sum()
                        })
                        .unwrap_or(0);
                    cache.insert((curr, with_dac, with_fft), res);
                    res
                }
            }
        }
    }

    let mut cache = HashMap::new();
    recurs_dfs(devices, &mut cache, "svr", false, false)
}

fn parse_input(input: &str) -> Result<Devices, Box<dyn Error>> {
    input
        .lines()
        .map(|line| {
            let (name, list) = sscanf!(line, "{str}: {str}")?;
            let outputs = list.split_whitespace().map(|d| d.to_owned()).collect_vec();
            Ok((name.to_owned(), outputs))
        })
        .collect::<Result<Devices, _>>()
}

type Devices = HashMap<String, Vec<String>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_devices = parse_input(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        )
        .unwrap();
        assert_eq!(part_1(&test_devices), 5);
    }

    #[test]
    fn test_part_2() {
        let test_devices = parse_input(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        )
        .unwrap();
        assert_eq!(part_2(&test_devices), 2);
    }
}
