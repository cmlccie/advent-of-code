use crate::get_input;
use std::collections::HashMap;
use std::iter::zip;
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 1: Historian Hysteria
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let (mut left_list, mut right_list) = parse_input(input);

    left_list.sort();
    right_list.sort();

    let total_distance: Distance = zip(left_list, right_list)
        .map(|(left, right)| (right - left).abs())
        .sum();

    Some(total_distance.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let (left_list, right_list) = parse_input(input);

    let right_list_id_count: HashMap<LocationID, IdCount> =
        right_list.iter().fold(HashMap::new(), |mut acc, id| {
            *acc.entry(*id).or_insert(0) += 1;
            acc
        });

    let similarity_score: SimilarityScore = left_list
        .iter()
        .map(|value| value * *right_list_id_count.get(value).unwrap_or(&0))
        .sum();

    Some(similarity_score.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type LocationID = i32;
type IdCount = i32;
type Distance = i32;
type SimilarityScore = i32;

fn parse_input(input: &str) -> (Vec<LocationID>, Vec<LocationID>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split("   ");
            let left: LocationID = parts.next().unwrap().parse().unwrap();
            let right: LocationID = parts.next().unwrap().parse().unwrap();
            (left, right)
        })
        .unzip()
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 1: Historian Hysteria")]
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
            part1(&get_input("../data/day1/example.txt")),
            get_answer("../data/day1/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2(&get_input("../data/day1/example.txt")),
            get_answer("../data/day1/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day1/input.txt")),
            get_answer("../data/day1/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day1/input.txt")),
            get_answer("../data/day1/input-part2-answer.txt")
        );
    }
}
