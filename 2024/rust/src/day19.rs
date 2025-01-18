use crate::get_input;
use cached::proc_macro::cached;
use regex::Regex;
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 19: Linen Layout
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let (patterns, designs) = parse_input(input);

    let pattern_regex = Regex::new(format!("^({})+$", patterns.join("|")).as_str()).unwrap();

    let possible_designs_count = designs
        .iter()
        .filter(|design| pattern_regex.is_match(design))
        .count();

    Some(possible_designs_count.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let (patterns, designs) = parse_input(input);

    let all_possible_design_combinations_count = designs
        .into_iter()
        .map(|design| count_ways_to_make_design(design, patterns.clone()))
        .sum::<DesignCount>();

    Some(all_possible_design_combinations_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Pattern = String;
type Patterns = Vec<Pattern>;
type Design = String;
type Designs = Vec<Design>;
type DesignCount = u64;

fn parse_input(input: &str) -> (Patterns, Designs) {
    let patterns = input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    let designs = input.lines().skip(2).map(|s| s.to_string()).collect();

    (patterns, designs)
}

#[cached]
fn count_ways_to_make_design(design: String, patterns: Patterns) -> DesignCount {
    // Base case
    if design.is_empty() {
        return 1;
    };

    // Recursive cases
    patterns
        .iter()
        .map(|pattern| {
            if design.starts_with(pattern) {
                count_ways_to_make_design(design[pattern.len()..].to_string(), patterns.clone())
            } else {
                0
            }
        })
        .sum()
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 19: Linen Layout")]
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
    fn test_example_part1() {
        assert_eq!(
            part1(&get_input("../data/day19/example.txt")),
            get_answer("../data/day19/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day19/input.txt")),
            get_answer("../data/day19/input-part1-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day19/input.txt")),
            get_answer("../data/day19/input-part2-answer.txt")
        );
    }
}
