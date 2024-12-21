use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

/*-------------------------------------------------------------------------------------------------
  Day 19: Linen Layout
-------------------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let (patterns, designs) = parse_input_file(input);

    let pattern_regex = Regex::new(format!("^({})+$", patterns.join("|")).as_str()).unwrap();

    let possible_designs_count = designs
        .iter()
        .filter(|design| pattern_regex.is_match(design))
        .count();

    possible_designs_count.to_string()
}

pub fn part2<P: AsRef<Path> + ?Sized>(_input: &P) -> String {
    unimplemented!()
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Pattern = String;
type Patterns = Vec<Pattern>;
type Design = String;
type Designs = Vec<Design>;

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

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1("../data/day19/example.txt"),
            solution("../data/day19/example-part1-answer.txt")
        );
    }

    // #[test]
    // fn test_part1_solution() {
    //     assert_eq!(
    //         part1("../data/day19/input.txt"),
    //         solution("../data/day19/input-part1-answer.txt")
    //     );
    // }

    // #[test]
    // fn test_part2_solution() {
    //     assert_eq!(
    //         part2("../data/day19/input.txt"),
    //         solution("../data/day19/input-part2-answer.txt")
    //     );
    // }
}
