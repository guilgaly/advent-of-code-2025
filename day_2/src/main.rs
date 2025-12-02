use common::time_execution;
use std::error::Error;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    // e.g. read lines
    let product_ranges = parse_input(&INPUT)?;

    time_execution("Part 1", || part_1(&product_ranges));

    time_execution("Part 2", || part_2(&product_ranges));

    Ok(())
}

fn part_1(product_ranges: &[Range]) -> u64 {
    sum_invalid_ids(product_ranges, |id| {
        let (x, y) = id.split_at(id.len() / 2);
        x == y
    })
}

fn part_2(product_ranges: &[Range]) -> u64 {
    fn is_id_repeating(id: &str, chunk_size: usize) -> bool {
        if id.len() % chunk_size != 0 {
            return false;
        }
        for i in 0..(id.len() / chunk_size - 1) {
            let start_0 = i * chunk_size;
            let end_0 = start_0 + chunk_size;
            let start_1 = (i + 1) * chunk_size;
            let end_1 = start_1 + chunk_size;
            if id[start_0..end_0] != id[start_1..end_1] {
                return false;
            }
        }
        true
    }

    sum_invalid_ids(product_ranges, |id| {
        let max_seq_length = id.len() / 2;
        for chunk_size in 1..=max_seq_length {
            if is_id_repeating(&id, chunk_size) {
                return true;
            }
        }
        false
    })
}

fn sum_invalid_ids<P>(product_ranges: &[Range], is_invalid: P) -> u64
where
    P: Fn(&str) -> bool,
{
    product_ranges
        .iter()
        .flat_map(|range| range.0..=range.1)
        .filter(|id| is_invalid(&id.to_string()))
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Range>, Box<dyn Error>> {
    input
        .split(",")
        .map(|range| {
            let mut split_range = range.split("-");
            let start = split_range
                .next()
                .ok_or(format!("Missing first part in range {}", range))?
                .parse::<u64>()?;
            let end = split_range
                .next()
                .ok_or(format!("Missing second part in range {}", range))?
                .parse::<u64>()?;
            Ok(Range(start, end))
        })
        .collect()
}

struct Range(u64, u64);

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: [Range; 11] = [
        Range(11, 22),
        Range(95, 115),
        Range(998, 1012),
        Range(1188511880, 1188511890),
        Range(222220, 222224),
        Range(1698522, 1698528),
        Range(446443, 446449),
        Range(38593856, 38593862),
        Range(565653, 565659),
        Range(824824821, 824824827),
        Range(2121212118, 2121212124),
    ];

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_INPUT), 1227775554);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_INPUT), 4174379265);
    }
}
