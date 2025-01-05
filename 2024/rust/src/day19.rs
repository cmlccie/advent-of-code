use cached::proc_macro::cached;
use regex::Regex;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

/*-------------------------------------------------------------------------------------------------
  Day 19: Linen Layout
-------------------------------------------------------------------------------------------------*/

fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let (patterns, designs) = parse_input_file(input);

    let pattern_regex = Regex::new(format!("^({})+$", patterns.join("|")).as_str()).unwrap();

    let possible_designs_count = designs
        .iter()
        .filter(|design| pattern_regex.is_match(design))
        .count();

    Some(possible_designs_count.to_string())
}

fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let (patterns, designs) = parse_input_file(input);

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

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> (Patterns, Designs) {
    let input = read_to_string(input).unwrap();

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
    fn test_example_part1() {
        assert_eq!(
            part1("../data/day19/example.txt"),
            answer("../data/day19/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day19/input.txt"),
            answer("../data/day19/input-part1-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day19/input.txt"),
            answer("../data/day19/input-part2-answer.txt")
        );
    }
}
