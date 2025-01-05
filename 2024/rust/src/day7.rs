use std::fs::read_to_string;
use std::path::{Path, PathBuf};

/*-------------------------------------------------------------------------------------------------
  Day 7: Bridge Repair
-------------------------------------------------------------------------------------------------*/

fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let calibration_equations = parse_input_file(input);

    let total_calibration_results = calibration_equations
        .iter()
        .filter(|(result, terms)| validate_equation(*result, terms, None, &[add, multiply]))
        .map(|(result, _)| result)
        .sum::<i64>();

    Some(total_calibration_results.to_string())
}

fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let calibration_equations = parse_input_file(input);

    let total_calibration_result = calibration_equations
        .iter()
        .filter(|(result, terms)| {
            validate_equation(*result, terms, None, &[add, multiply, concatenate])
        })
        .map(|(result, _)| result)
        .sum::<i64>();

    Some(total_calibration_result.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Vec<(i64, Vec<i64>)> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(&[':', ' ']);
            let result: i64 = split.next().unwrap().parse().unwrap();
            let terms: Vec<i64> = split.skip(1).map(|x| x.parse().unwrap()).collect();
            (result, terms)
        })
        .collect()
}

fn validate_equation(
    result: i64,
    terms: &[i64],
    acc: Option<i64>,
    operators: &[fn(Option<i64>, i64) -> i64],
) -> bool {
    // Base case
    if terms.is_empty() {
        return result == acc.unwrap();
    };

    // Recursive cases
    operators
        .iter()
        .any(|op| validate_equation(result, &terms[1..], Some(op(acc, terms[0])), operators))
}

/*-----------------------------------------------------------------------------
  Operators
-----------------------------------------------------------------------------*/

fn add(acc: Option<i64>, term: i64) -> i64 {
    acc.unwrap_or(0) + term
}

fn multiply(acc: Option<i64>, term: i64) -> i64 {
    acc.unwrap_or(1) * term
}

fn concatenate(acc: Option<i64>, term: i64) -> i64 {
    match acc {
        Some(acc) => (acc * 10u64.pow(term.ilog10() + 1) as i64) + term,
        None => term,
    }
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 7: Bridge Repair")]
pub enum Args {
    Part1 { input: PathBuf },
    Part2 { input: PathBuf },
}

pub fn main(args: Args) -> Option<String> {
    match args {
        Args::Part1 { input } => part1(&input),
        Args::Part2 { input } => part2(&input),
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::answers::answer;

    #[test]
    fn test_example_solution_part1() {
        assert_eq!(
            part1("../data/day7/example.txt"),
            answer("../data/day7/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2("../data/day7/example.txt"),
            answer("../data/day7/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day7/input.txt"),
            answer("../data/day7/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day7/input.txt"),
            answer("../data/day7/input-part2-answer.txt")
        );
    }
}
