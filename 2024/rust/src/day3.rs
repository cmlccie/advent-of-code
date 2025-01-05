use crate::shared::inputs::get_input;
use regex::Regex;
use std::path::PathBuf;
use std::sync::OnceLock;

/*-------------------------------------------------------------------------------------------------
  Day 3: Mull It Over
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let mul_regex = get_mul_regex();

    let multiplication_sum = mul_regex
        .captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i64>().unwrap();
            let b = cap[2].parse::<i64>().unwrap();
            a * b
        })
        .sum::<i64>();

    Some(multiplication_sum.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let enabled_sections: Vec<_> = input
        .split("do()")
        .map(|section| section.split("don't()").next())
        .collect();

    let mul_regex = get_mul_regex();

    let enabled_multiplications_sum = enabled_sections.iter().fold(0, |acc, section| {
        acc + mul_regex
            .captures_iter(section.unwrap())
            .map(|cap| {
                let a = cap[1].parse::<i64>().unwrap();
                let b = cap[2].parse::<i64>().unwrap();
                a * b
            })
            .sum::<i64>()
    });

    Some(enabled_multiplications_sum.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

static MUL_REGEX_CELL: OnceLock<Regex> = OnceLock::new();

fn get_mul_regex() -> &'static Regex {
    MUL_REGEX_CELL.get_or_init(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap())
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 3: Mull It Over")]
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
    fn test_example_solution_part1() {
        assert_eq!(
            part1(&get_input("../data/day3/example-part1.txt")),
            get_answer("../data/day3/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2(&get_input("../data/day3/example-part2.txt")),
            get_answer("../data/day3/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day3/input.txt")),
            get_answer("../data/day3/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day3/input.txt")),
            get_answer("../data/day3/input-part2-answer.txt")
        );
    }
}
