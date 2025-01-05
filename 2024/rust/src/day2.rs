use crate::shared::inputs::get_input;
use crate::shared::logging::log_if_error;
use anyhow::{anyhow, Result};
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 2: Red-Nosed Reports
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let safe_report_count = input
        .lines()
        .map(parse_line)
        .inspect(log_if_error)
        .filter_map(Result::ok)
        .map(|report| report_status(&report))
        .map(|status| match status {
            ReportStatus::Safe => 1,
            ReportStatus::Unsafe => 0,
        })
        .sum::<i64>();

    Some(safe_report_count.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let updated_safe_report_count = input
        .lines()
        .map(parse_line)
        .inspect(log_if_error)
        .filter_map(Result::ok)
        .map(|report| report_status_with_problem_dampener(&report))
        .map(|status| match status {
            ReportStatus::Safe => 1,
            ReportStatus::Unsafe => 0,
        })
        .sum::<i64>();

    Some(updated_safe_report_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

fn parse_line(line: &str) -> Result<Vec<i64>> {
    if line.is_empty() {
        return Err(anyhow!("Empty line"));
    }

    line.split(' ')
        .map(|s| s.parse::<i64>().map_err(|e| e.into()))
        .collect()
}

#[derive(Debug, PartialEq)]
enum ReportStatus {
    Safe,
    Unsafe,
}

fn report_all_increasing(report: &[i64]) -> bool {
    report.windows(2).all(|levels| levels[0] < levels[1])
}

fn report_all_decreasing(report: &[i64]) -> bool {
    report.windows(2).all(|levels| levels[0] > levels[1])
}

fn report_difference_tolerance(report: &[i64], min: i64, max: i64) -> bool {
    report.windows(2).all(|levels| {
        let difference = (levels[0] - levels[1]).abs();
        (min <= difference) && (difference <= max)
    })
}

fn report_status(report: &[i64]) -> ReportStatus {
    if (report_all_increasing(report) || report_all_decreasing(report))
        && report_difference_tolerance(report, 1, 3)
    {
        ReportStatus::Safe
    } else {
        ReportStatus::Unsafe
    }
}

fn report_status_with_problem_dampener(report: &[i64]) -> ReportStatus {
    if report_status(report) == ReportStatus::Safe {
        return ReportStatus::Safe;
    };

    // Problem dampener: Remove one level at a time and check if the report is safe
    for remove_level in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(remove_level);

        if report_status(&modified_report) == ReportStatus::Safe {
            return ReportStatus::Safe;
        }
    }

    ReportStatus::Unsafe
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 2: Red-Nosed Reports")]
pub enum Args {
    Part1 { input: PathBuf },
    Part2 { input: PathBuf },
}

pub fn main(args: Args) -> Option<String> {
    match args {
        Args::Part1 { input } => part1(&get_input(&input)),
        Args::Part2 { input } => part2(&get_input(&input)),
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::answers::get_answer;

    #[test]
    fn test_parse_line() {
        let file_contents = get_input("../data/day2/example.txt");
        let mut parsed_lines = file_contents.lines().map(parse_line).filter_map(Result::ok);

        assert_eq!(parsed_lines.next().unwrap(), vec![7, 6, 4, 2, 1]);
        assert_eq!(parsed_lines.next().unwrap(), vec![1, 2, 7, 8, 9]);
        assert_eq!(parsed_lines.next().unwrap(), vec![9, 7, 6, 2, 1]);
        assert_eq!(parsed_lines.next().unwrap(), vec![1, 3, 2, 4, 5]);
        assert_eq!(parsed_lines.next().unwrap(), vec![8, 6, 4, 4, 1]);
        assert_eq!(parsed_lines.next().unwrap(), vec![1, 3, 6, 7, 9]);
        assert!(parsed_lines.next().is_none());
    }

    #[test]
    fn test_example_solution_part1() {
        assert_eq!(
            part1(&get_input("../data/day2/example.txt")),
            get_answer("../data/day2/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2(&get_input("../data/day2/example.txt")),
            get_answer("../data/day2/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day2/input.txt")),
            get_answer("../data/day2/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day2/input.txt")),
            get_answer("../data/day2/input-part2-answer.txt")
        );
    }
}
