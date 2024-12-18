/*-------------------------------------------------------------------------------------------------
  Day 3: Mull It Over
-------------------------------------------------------------------------------------------------*/

use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::OnceLock;

/*--------------------------------------------------------------------------------------
  Part 1
--------------------------------------------------------------------------------------*/

static MUL_REGEX_CELL: OnceLock<Regex> = OnceLock::new();

fn get_mul_regex() -> &'static Regex {
    MUL_REGEX_CELL.get_or_init(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap())
}

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let input_text = read_to_string(input).unwrap();

    let mul_regex = get_mul_regex();

    mul_regex
        .captures_iter(&input_text)
        .map(|cap| {
            let a = cap[1].parse::<i64>().unwrap();
            let b = cap[2].parse::<i64>().unwrap();
            a * b
        })
        .sum::<i64>()
        .to_string()
}

/*--------------------------------------------------------------------------------------
  Part 2
--------------------------------------------------------------------------------------*/

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let input_text = read_to_string(input).unwrap();

    let enabled_sections: Vec<_> = input_text
        .split("do()")
        .map(|section| section.split("don't()").next())
        .collect();

    let mul_regex = get_mul_regex();

    enabled_sections
        .iter()
        .fold(0, |acc, section| {
            acc + mul_regex
                .captures_iter(section.unwrap())
                .map(|cap| {
                    let a = cap[1].parse::<i64>().unwrap();
                    let b = cap[2].parse::<i64>().unwrap();
                    a * b
                })
                .sum::<i64>()
        })
        .to_string()
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_example_solution_part1() {
        assert_eq!(
            part1("../data/day3/example-part1.txt"),
            solution("../data/day3/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2("../data/day3/example-part2.txt"),
            solution("../data/day3/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day3/input.txt"),
            solution("../data/day3/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day3/input.txt"),
            solution("../data/day3/input-part2-answer.txt")
        );
    }
}
