use crate::get_input;
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 2: Red-Nosed Reports
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let safe_report_count = input
        .lines()
        .map(parse_line)
        .map(|report| report_status(&report))
        .filter(|&status| status)
        .count();

    Some(safe_report_count.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let updated_safe_report_count = input
        .lines()
        .map(parse_line)
        .map(|report| report_status_with_problem_dampener(&report))
        .filter(|&status| status)
        .count();

    Some(updated_safe_report_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Level = i8;

fn parse_line(line: &str) -> Vec<Level> {
    line.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn report_all_increasing(report: &[Level]) -> bool {
    report.windows(2).all(|levels| levels[0] < levels[1])
}

fn report_all_decreasing(report: &[Level]) -> bool {
    report.windows(2).all(|levels| levels[0] > levels[1])
}

fn report_difference_tolerance(report: &[Level], min: Level, max: Level) -> bool {
    report.windows(2).all(|levels| {
        let difference = (levels[0] - levels[1]).abs();
        (min..=max).contains(&difference)
    })
}

fn report_status(report: &[Level]) -> bool {
    (report_all_increasing(report) || report_all_decreasing(report))
        && report_difference_tolerance(report, 1, 3)
}

fn report_status_with_problem_dampener(report: &[Level]) -> bool {
    if report_status(report) {
        return true;
    };

    // Problem dampener: Remove one level at a time and check if the report is safe
    for remove_level in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(remove_level);

        if report_status(&modified_report) {
            return true;
        }
    }

    false
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
    use crate::get_answer;

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
