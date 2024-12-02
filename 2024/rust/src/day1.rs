/*-------------------------------------------------------------------------------------------------
  Day 1: Historian Hysteria
-------------------------------------------------------------------------------------------------*/

use crate::utils::log_if_error;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

/*--------------------------------------------------------------------------------------
  Helper Functions
--------------------------------------------------------------------------------------*/

fn parse_line(line: &str) -> Result<(i64, i64)> {
    if line.is_empty() {
        return Err(anyhow!("Empty line"));
    }
    let mut parts = line.split("   ");
    let left = parts
        .next()
        .ok_or(anyhow!("Missing left value"))?
        .parse::<i64>()?;
    let right = parts
        .next()
        .ok_or(anyhow!("Missing right value"))?
        .parse::<i64>()?;
    Ok((left, right))
}

fn parse_file<P: AsRef<Path> + ?Sized>(file_path: &P) -> (Vec<i64>, Vec<i64>) {
    read_to_string(file_path)
        .unwrap()
        .lines()
        .map(parse_line)
        .inspect(log_if_error)
        .filter_map(Result::ok)
        .unzip()
}

/*--------------------------------------------------------------------------------------
  Part 1
--------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> i64 {
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = parse_file(input);

    left_list.sort();
    right_list.sort();

    // Total difference between the two lists
    std::iter::zip(left_list.iter(), right_list.iter())
        .map(|(left, right)| (right - left).abs())
        .sum()
}

/*--------------------------------------------------------------------------------------
  Part 2
--------------------------------------------------------------------------------------*/

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> i64 {
    let (left_list, right_list): (Vec<_>, Vec<_>) = parse_file(input);

    // Calculate similarity score
    let right_list_id_count: HashMap<i64, i64> =
        right_list.iter().fold(HashMap::new(), |mut acc, id| {
            *acc.entry(*id).or_insert(0) += 1;
            acc
        });

    left_list
        .iter()
        .map(|value| value * *right_list_id_count.get(value).unwrap_or(&0))
        .sum()
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_parse_line() {
        assert!(parse_line("").is_err());
        assert!(parse_line("1").is_err());
        assert!(parse_line("   2").is_err());

        assert_eq!(parse_line("1   2").unwrap(), (1, 2));
    }

    #[test]
    fn test_example_solution_part1() {
        assert_eq!(
            part1("../data/day1/example.txt"),
            solution("../data/day1/example-solution-part1.txt").unwrap()
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2("../data/day1/example.txt"),
            solution("../data/day1/example-solution-part2.txt").unwrap()
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day1/input.txt"),
            solution("../data/day1/solution-part1.txt").unwrap()
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day1/input.txt"),
            solution("../data/day1/solution-part2.txt").unwrap()
        );
    }
}